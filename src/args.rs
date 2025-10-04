// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

use clap::{Args, Parser, Subcommand};

use crate::print::PrintOneLine;

const AFTER_LONG_HELP: &str = "\
Automatically print colored output if stdout is a TTY, unless overridden by
environment variables as follows:

- If $NO_COLOR is set to a non-empty string, never print any colors.
- If $CLICOLOR_FORCE is set to a non-empty string, always print colors even if
  stdout is not a TTY.
";

const LONG_VERSION: &str = concat!(
    env!("CARGO_BIN_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    "\n",
    env!("CARGO_PKG_HOMEPAGE"),
    "\nLicense: ",
    env!("CARGO_PKG_LICENSE"),
);

/// Analyse pacman dependency graphs.
#[derive(Debug, Parser)]
#[command(version, about, after_long_help = AFTER_LONG_HELP, long_version = LONG_VERSION)]
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

impl Orphans {
    pub fn oneline_style(&self) -> PrintOneLine {
        if self.quiet {
            PrintOneLine::NameOnly
        } else {
            PrintOneLine::WithVersion
        }
    }
}
