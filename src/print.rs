// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

//! Utilities for printing packages.

use std::io::prelude::*;

use anstyle::{AnsiColor, Reset, Style};
use pacgraph::graph::{DependencyEdge, PackageNode};
use petgraph::{
    dot::{Config, Dot, RankDir},
    visit::{
        Data, EdgeRef, GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable, NodeRef,
    },
};

/// How to print a package.
#[derive(Debug, Copy, Clone)]
pub enum PrintOneLine {
    /// Only print the name.
    NameOnly,
    /// Print with version.
    WithVersion,
}

/// Print a package on one single line.
pub fn print_package_one_line<W: Write>(
    write: &mut W,
    package: &alpm::Package,
    how: PrintOneLine,
) -> Result<(), std::io::Error> {
    match how {
        PrintOneLine::NameOnly => writeln!(write, "{}", package.name()),
        PrintOneLine::WithVersion => {
            let bold = Style::new().bold();
            let green = bold.fg_color(Some(AnsiColor::Green.into()));
            writeln!(
                write,
                "{bold}{} {green}{}{Reset}",
                package.name(),
                package.version()
            )
        }
    }
}

/// Print a package graph as dot.
pub fn print_package_graph<'a, G, W: Write>(
    write: &mut W,
    graph: G,
    format: PrintOneLine,
) -> std::io::Result<()>
where
    G: GraphProp
        + Data<NodeWeight = PackageNode<'a>, EdgeWeight = DependencyEdge>
        + IntoEdgeReferences
        + IntoNodeReferences
        + NodeIndexable,
{
    let get_node_attributes = |_graph, node: G::NodeRef| {
        let package = node.weight();
        match format {
            PrintOneLine::NameOnly => format!(
                "label = <<FONT FACE=\"sans-serif\">{}</FONT>>",
                package.name()
            ),
            PrintOneLine::WithVersion => format!(
                "label = <<FONT FACE=\"sans-serif\"><B>{name} <FONT COLOR=\"green\">{version}</FONT></B></FONT>>",
                name = package.name(),
                version = package.version()
            ),
        }
    };
    let dot = Dot::with_attr_getters(
        graph,
        &[
            Config::EdgeNoLabel,
            Config::NodeNoLabel,
            Config::RankDir(RankDir::TB),
        ],
        &|_graph, edge| match *edge.weight() {
            DependencyEdge::Required => "style = solid".to_string(),
            DependencyEdge::Optional => "style = dashed".to_string(),
        },
        &get_node_attributes,
    );
    writeln!(write, "{dot}")
}
