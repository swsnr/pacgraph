// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the EUPL-1.2 OR GPL-3.0
//
// See https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12

//! Utilities for ALPM.

use alpm::LogLevel;
use tracing::Level;

/// Tracing log callback for [`alpm::Alpm::set_log_cb`].
///
/// Use with [`alpm::Alpm::set_log_cb`] to log `message` to [`tracing`], to the
/// `alpm` target, at a level equivalent to the given ALPM `level`.
pub fn tracing_log_cb(level: alpm::LogLevel, message: &str, _: &mut ()) {
    match level {
        LogLevel::DEBUG => tracing::event!(target: "alpm", Level::DEBUG, "{}", message),
        LogLevel::WARNING => tracing::event!(target: "alpm", Level::WARN, "{}", message),
        LogLevel::ERROR => tracing::event!(target: "alpm", Level::ERROR, "{}", message),
        _ => tracing::event!(target: "alpm", Level::TRACE, "{}", message),
    }
}
