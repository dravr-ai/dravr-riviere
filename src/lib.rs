// ABOUTME: Root module for the dravr-riviere time-series storage engine
// ABOUTME: Re-exports all public types for metric storage, querying, aggregation, and archival
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! # dravr-riviere
//!
//! Postgres-backed time-series storage engine for health and fitness metrics.
//! Provides a trait-based storage interface with windowed aggregation, range
//! queries, and a comprehensive catalog of 90+ metric types following Open
//! Wearables ID conventions.

/// Windowed aggregation types and functions.
pub mod aggregation;
/// Daily archive model for data lifecycle.
pub mod archive;
/// Structured error types.
pub mod error;
/// `MetricKey` trait for generic metric identification.
pub mod key;
/// `DataPoint` timestamped value type.
pub mod point;
/// `TimeRange` and `QueryResult` query types.
pub mod query;
/// `TimeSeries` sorted collection of data points.
pub mod series;
/// `SeriesType` enum with 90+ health/fitness metric type catalog.
pub mod series_type;
/// `TimeSeriesStore` trait and `InMemoryStore` implementation.
pub mod store;

// Re-export primary types at crate root for convenience.
pub use aggregation::{AggregatedPoint, Aggregation};
pub use archive::DataPointArchive;
pub use error::{Result, RiviereError};
pub use key::MetricKey;
pub use point::DataPoint;
pub use query::{QueryResult, TimeRange};
pub use series::TimeSeries;
pub use series_type::{SeriesType, SeriesTypeDefinition, ALL_SERIES_TYPES};
pub use store::{InMemoryStore, TimeSeriesStore};
