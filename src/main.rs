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

use alpm::Alpm;
use alpm_utils::{alpm_with_conf, config::Config};
use clap::Parser;
use pacgraph::graph::DependencyEdge;
use petgraph::visit::{EdgeFiltered, EdgeRef};

use crate::{args::CliArgs, print::print_package_one_line};

mod args;
mod print;

fn list_orphans(options: &args::Orphans, alpm: &Alpm) -> std::io::Result<()> {
    let localdb = alpm.localdb();
    let pkg_graph = pacgraph::graph::build_graph_for_localdb(localdb);
    let mut orphans = if options.ignore_optdepends {
        pacgraph::dependencies::orphans(&EdgeFiltered::from_fn(&pkg_graph, |edge| {
            *edge.weight() == DependencyEdge::Required
        }))
        .collect::<Vec<_>>()
    } else {
        pacgraph::dependencies::orphans(&pkg_graph).collect::<Vec<_>>()
    };
    // Sort alphabetically
    orphans.sort_by_key(|pkg| pkg.name());

    let how = if options.quiet {
        print::PrintOneLine::NameOnly
    } else {
        print::PrintOneLine::WithVersion
    };
    for pkg in orphans {
        print_package_one_line(&mut std::io::stdout(), pkg, how)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let args = CliArgs::parse();

    let config = Config::new().map_err(|error| match error.kind {
        alpm_utils::config::ErrorKind::Io(error) => error,
        _ => std::io::Error::new(std::io::ErrorKind::InvalidData, error),
    })?;
    let alpm = alpm_with_conf(&config).map_err(std::io::Error::other)?;
    alpm.set_log_cb((), pacgraph::alpm::tracing_log_cb);

    match args.command {
        args::Command::Orphans(orphans) => list_orphans(&orphans, &alpm)?,
    }

    Ok(())
}
