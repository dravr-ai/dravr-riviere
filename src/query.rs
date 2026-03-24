// ABOUTME: TimeRange and QueryResult types for time-series range queries
// ABOUTME: Defines inclusive start / exclusive end query bounds and structured results
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::point::DataPoint;

/// Half-open time range `[start, end)` used to bound queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Inclusive lower bound.
    pub start: DateTime<Utc>,
    /// Exclusive upper bound.
    pub end: DateTime<Utc>,
}

impl TimeRange {
    /// Create a new time range.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `start > end`.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        debug_assert!(start <= end, "TimeRange start must not exceed end");
        Self { start, end }
    }

    /// Test whether `ts` falls within `[start, end)`.
    pub fn contains(&self, ts: DateTime<Utc>) -> bool {
        ts >= self.start && ts < self.end
    }

    /// Duration of the range in seconds.
    pub fn duration_secs(&self) -> i64 {
        (self.end - self.start).num_seconds()
    }
}

/// Structured result of a range query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Data points matching the query.
    pub points: Vec<DataPoint>,
    /// Total number of matching points (may differ from `points.len()` when paginated).
    pub total_count: usize,
}

impl QueryResult {
    /// Build a result from a complete set of points (no pagination).
    pub fn from_points(points: Vec<DataPoint>) -> Self {
        let total_count = points.len();
        Self {
            points,
            total_count,
        }
    }
}
