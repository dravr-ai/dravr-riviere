// ABOUTME: Structured error types for the dravr-riviere time-series storage engine
// ABOUTME: Uses thiserror for ergonomic error handling with typed variants
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

/// Errors produced by the riviere time-series storage engine.
#[derive(Debug, thiserror::Error)]
pub enum RiviereError {
    /// The requested metric series was not found.
    #[error("series not found: source={source_id}, type={series_type}")]
    SeriesNotFound {
        /// Identifier of the data source (e.g. device or user ID).
        source_id: String,
        /// Numeric series type identifier.
        series_type: u32,
    },

    /// A query contained invalid parameters.
    #[error("invalid query: {reason}")]
    InvalidQuery {
        /// Human-readable explanation of why the query is invalid.
        reason: String,
    },

    /// A data point contained an invalid value.
    #[error("invalid data point: {reason}")]
    InvalidDataPoint {
        /// Human-readable explanation of why the data point is invalid.
        reason: String,
    },

    /// An unknown series type identifier was provided.
    #[error("unknown series type id: {id}")]
    UnknownSeriesType {
        /// The unrecognised numeric identifier.
        id: u32,
    },

    /// An internal storage error occurred.
    #[error("storage error: {message}")]
    Storage {
        /// Description of the storage failure.
        message: String,
    },
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, RiviereError>;
