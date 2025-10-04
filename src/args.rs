// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

use clap::{Args, Parser, Subcommand};

/// Analyse pacman dependency graphs.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Orphans(Orphans),
}

/// List orphan packages.
#[derive(Args, Debug)]
pub struct Orphans {
    /// Show less information.
    #[clap(short = 'q', long = "quiet")]
    pub quiet: bool,
    #[clap(long)]
    /// Ignore optional dependencies.
    pub ignore_optdepends: bool,
}
