// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

//! Graphs of ALPM packages.

use std::{fmt::Display, hash::Hash, ops::Deref};

use alpm::{Db, Package, Pkg};
use petgraph::prelude::DiGraphMap;
use tracing::{debug, debug_span, warn};

/// The weight of a dependency edge.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DependencyEdge {
    /// A required dependency.
    Required,
    /// An optional dependency.
    Optional,
}

/// A package node in a graph.
///
/// Wrap a reference to an [`alpm::Package`], which implements equality, ordering,
/// and hashing based on the pointer to the underlying `alpm_pkg_t` structure.
/// This allows using this struct as a node identifier in a
/// [`petgraph::prelude::DiGraphMap`] without an extra level of node indexing.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PackageNode<'a>(&'a Package);

impl<'a> PackageNode<'a> {
    /// Get the package of this node.
    #[must_use]
    pub fn package(self) -> &'a Package {
        self.0
    }
}

impl Display for PackageNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
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

/// An ALPM dependency graph.
pub type AlpmDepGraphMap<'a> = DiGraphMap<PackageNode<'a>, DependencyEdge>;

/// Build a dependency graph for the local database.
///
/// Build a dependency graph for the local ALPM database, which follows the
/// `required_by` and `optional_for` edges.  This only works for the local
/// database, but guarantees to return resolvable dependencies, so the returned
/// graph is complete.
pub fn build_graph_for_localdb(db: &Db) -> AlpmDepGraphMap<'_> {
    let mut g = DiGraphMap::new();
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
