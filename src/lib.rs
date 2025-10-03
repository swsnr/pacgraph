// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

//! Graph analysis tools for ALPM packages.

#![deny(warnings, missing_docs, missing_debug_implementations, clippy::all, clippy::pedantic,
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

pub mod alpm;
pub mod dependencies;
pub mod graph;
