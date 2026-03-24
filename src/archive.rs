// ABOUTME: DataPointArchive model for daily pre-aggregated time-series data
// ABOUTME: Supports data lifecycle by storing rolled-up values per bucket for long-term retention
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::aggregation::Aggregation;

/// A pre-aggregated data point representing a daily (or other bucket) rollup.
///
/// Archives allow the storage engine to compact raw data points into summary
/// records for long-term retention while keeping query performance high.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPointArchive {
    /// Identifier of the data source (e.g. device or user ID).
    pub source_id: String,
    /// Numeric series type identifier.
    pub series_type_id: u32,
    /// Start of the aggregation bucket (e.g. midnight UTC for daily buckets).
    pub bucket_start: DateTime<Utc>,
    /// The aggregation function used to produce this value.
    pub aggregation_type: Aggregation,
    /// The aggregated value.
    pub value: f64,
    /// Number of raw data points that were rolled up into this archive entry.
    pub sample_count: u32,
}

impl DataPointArchive {
    /// Create a new archive entry.
    pub fn new(
        source_id: String,
        series_type_id: u32,
        bucket_start: DateTime<Utc>,
        aggregation_type: Aggregation,
        value: f64,
        sample_count: u32,
    ) -> Self {
        Self {
            source_id,
            series_type_id,
            bucket_start,
            aggregation_type,
            value,
            sample_count,
        }
    }
}
