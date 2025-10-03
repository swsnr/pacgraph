// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

//! Utilities for printing packages.

use std::io::Write;

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
        PrintOneLine::WithVersion => writeln!(write, "{} {}", package.name(), package.version()),
    }
}
