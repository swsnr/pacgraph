// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

//! Analyse dependencies of ALPM packages.

use std::collections::{HashMap, VecDeque};

use alpm::PackageReason;
use petgraph::visit::{
    Bfs, GraphRef, IntoNeighbors, IntoNodeIdentifiers, NodeCount, VisitMap as _, Visitable,
};
use tracing::{debug, debug_span};

use crate::graph::PackageNode;

/// Iterate over all orphans in a dependency graph.
///
/// An orphan package is a package which is not transitively reachable from any
/// explicitly installed (see [`alpm::PackageReason`] and [`alpm::Pkg::reason`])
/// package.
///
/// The returned iterator iterates over orphans in undefined order.
pub fn orphans<'a, G>(graph: G) -> impl Iterator<Item = &'a alpm::Package>
where
    G: GraphRef
        + NodeCount
        + Visitable<NodeId = PackageNode<'a>>
        + IntoNeighbors
        + IntoNodeIdentifiers,
{
    #[allow(
        clippy::mutable_key_type,
        reason = "We do not mutate the package pointer while traversing the graph"
    )]
    let mut marked_pkgs = HashMap::with_capacity(graph.node_count());
    let explicit_pkgs = graph
        .node_identifiers()
        .filter(|p| p.reason() == PackageReason::Explicit);
    // We manually initialize BFS, because we'd like to retain the visit map
    // for all explicit packages, so as to avoid repeatedly traversing branches
    // that were already marked by another explicit package.
    let mut bfs = Bfs {
        discovered: graph.visit_map(),
        stack: VecDeque::new(),
    };
    for node in explicit_pkgs {
        bfs.stack.push_front(node);
        bfs.discovered.visit(node);
        debug!("Marking from {}", node.name());
        let _guard = debug_span!("mark-bfs", package = node.name()).entered();
        marked_pkgs.insert(node, true);
        let mut bfs = Bfs::new(&graph, node);
        while let Some(node) = bfs.next(&graph) {
            if Some(true) != marked_pkgs.insert(node, true) {
                debug!(package = node.name(), "Marking {}", node.name());
            }
        }
    }

    graph.node_identifiers().filter_map(move |pkg| {
        let is_marked = marked_pkgs.get(&pkg).copied().unwrap_or_default();
        (!is_marked).then_some(pkg.package())
    })
}
