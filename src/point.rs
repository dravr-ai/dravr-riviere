// ABOUTME: DataPoint struct representing a single timestamped numeric value
// ABOUTME: The fundamental unit of time-series data stored and queried by the engine
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single timestamped numeric measurement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPoint {
    /// UTC timestamp of the measurement.
    pub timestamp: DateTime<Utc>,
    /// Numeric value of the measurement.
    pub value: f64,
}

impl DataPoint {
    /// Create a new data point.
    pub fn new(timestamp: DateTime<Utc>, value: f64) -> Self {
        Self { timestamp, value }
    }
}

impl Eq for DataPoint {}

impl PartialOrd for DataPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DataPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
