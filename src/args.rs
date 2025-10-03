// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

use argh::FromArgs;

/// Analyse pacman dependency graphs.
#[derive(Debug, FromArgs)]
pub struct Args {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Orphans(Orphans),
}

/// List orphan packages.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "orphans")]
pub struct Orphans {
    #[argh(switch, short = 'q', description = "show less information")]
    pub quiet: bool,
}
