// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

#![deny(warnings, clippy::all, clippy::pedantic,
    // Do cfg(test) right
    clippy::cfg_not_test,
    clippy::tests_outside_test_module,
    // Guard against left-over debugging output
    clippy::dbg_macro,
    clippy::unimplemented,
    clippy::use_debug,
    clippy::todo,
    // Don't panic carelessly
    clippy::get_unwrap,
    clippy::unused_result_ok,
    clippy::unwrap_in_result,
    clippy::indexing_slicing,
    // Do not carelessly ignore errors
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    // Code smells
    clippy::float_cmp_const,
    clippy::string_to_string,
    clippy::if_then_some_else_none,
    clippy::large_include_file,
    // Disable as casts
    clippy::as_conversions,
)]
#![forbid(unsafe_code)]

use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Deref};

use alpm::{Alpm, Db, LogLevel, Package, PackageReason, Pkg};
use petgraph::{prelude::DiGraphMap, visit::Bfs};
use tracing::{Level, debug, debug_span, warn};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DependencyEdge {
    Required,
    Optional,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct PackageNode<'a>(&'a Package);

impl<'a> PackageNode<'a> {
    fn package(self) -> &'a Package {
        self.0
    }
}

impl Display for PackageNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.package().name())
    }
}

impl PartialEq for PackageNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for PackageNode<'_> {}

#[allow(
    clippy::non_canonical_partial_ord_impl,
    reason = "We forward to pointer impls and believe that these are canonical"
)]
impl PartialOrd for PackageNode<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ptr_self: *const Package = self.0;
        let ptr_other: *const Package = other.0;
        ptr_self.partial_cmp(&ptr_other)
    }
}

impl Ord for PackageNode<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ptr_self: *const Package = self.0;
        let ptr_other: *const Package = other.0;
        ptr_self.cmp(&ptr_other)
    }
}

impl Hash for PackageNode<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state);
    }
}

impl Deref for PackageNode<'_> {
    type Target = Pkg;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

fn build_graph(db: &Db) -> DiGraphMap<PackageNode<'_>, DependencyEdge> {
    let mut g = DiGraphMap::new();
    // Second pass: Edges
    for package in db.pkgs() {
        let _guard = debug_span!("package edges", package = package.name()).entered();
        debug!(
            package = package.name(),
            "Adding node for {}",
            package.name()
        );
        g.add_node(PackageNode(package));
        for requiree in package.required_by() {
            match db.pkg(requiree.as_str()) {
                Ok(requiree) => {
                    debug!(
                        package = requiree.name(),
                        "Adding required edge {} -> {}",
                        requiree.name(),
                        package.name()
                    );
                    g.add_edge(
                        PackageNode(requiree),
                        PackageNode(package),
                        DependencyEdge::Required,
                    );
                }
                Err(error) => {
                    warn!(
                        package = &requiree,
                        "Package {} is required by {requiree} which was not found in local database: {error}",
                        package.name(),
                    );
                }
            }
        }
        for opt_requiree in package.optional_for() {
            match db.pkg(opt_requiree.as_str()) {
                Ok(opt_requiree) => {
                    debug!(
                        package = opt_requiree.name(),
                        "Adding optional edge {} -> {}",
                        opt_requiree.name(),
                        package.name()
                    );
                    g.add_edge(
                        PackageNode(opt_requiree),
                        PackageNode(package),
                        DependencyEdge::Optional,
                    );
                }
                Err(error) => {
                    warn!(
                        package = &opt_requiree,
                        "Package {} is required by {opt_requiree} which was not found in local database: {error}",
                        package.name(),
                    );
                }
            }
        }
    }
    g
}

fn main() {
    tracing_subscriber::fmt::init();
    let alpm = Alpm::new("/", "/var/lib/pacman/").unwrap();
    alpm.set_log_cb((), |level, msg, ()| match level {
        LogLevel::DEBUG => tracing::event!(target: "alpm", Level::DEBUG, "{}", msg),
        LogLevel::WARNING => tracing::event!(target: "alpm", Level::WARN, "{}", msg),
        LogLevel::ERROR => tracing::event!(target: "alpm", Level::ERROR, "{}", msg),
        _ => tracing::event!(target: "alpm", Level::TRACE, "{}", msg),
    });

    let localdb = alpm.localdb();
    let pkg_graph = build_graph(localdb);
    #[allow(
        clippy::mutable_key_type,
        reason = "We do not mutate the package pointer while traversing the graph"
    )]
    let mut marked_pkgs = HashMap::with_capacity(pkg_graph.node_count());
    let explicit_pkgs = pkg_graph
        .nodes()
        .filter(|p| p.reason() == PackageReason::Explicit);
    for node in explicit_pkgs {
        debug!("Marking from {}", node.name());
        let _guard = debug_span!("mark-bfs", package = node.name()).entered();
        marked_pkgs.insert(node, true);
        let mut bfs = Bfs::new(&pkg_graph, node);
        while let Some(node) = bfs.next(&pkg_graph) {
            if Some(true) != marked_pkgs.insert(node, true) {
                debug!("Marking {}", node.name());
            }
        }
    }

    let mut orphans = pkg_graph
        .nodes()
        .filter_map(|pkg| {
            let is_marked = marked_pkgs.get(&pkg).copied().unwrap_or_default();
            (!is_marked).then_some(pkg.package())
        })
        .collect::<Vec<_>>();

    orphans.sort_by_key(|pkg| pkg.name());
    for pkg in orphans {
        println!("{}", pkg.name());
    }
}
