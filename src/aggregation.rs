// ABOUTME: Windowed aggregation types and functions for time-series rollups
// ABOUTME: Supports avg, min, max, sum, count, first, and last over configurable windows
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::point::DataPoint;

/// Aggregation function applied within each time window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aggregation {
    /// Arithmetic mean of values in the window.
    Avg,
    /// Minimum value in the window.
    Min,
    /// Maximum value in the window.
    Max,
    /// Sum of all values in the window.
    Sum,
    /// Number of data points in the window.
    Count,
    /// First (earliest) value in the window.
    First,
    /// Last (latest) value in the window.
    Last,
}

/// Result of aggregating a single time window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedPoint {
    /// Start of the aggregation window.
    pub window_start: DateTime<Utc>,
    /// End of the aggregation window.
    pub window_end: DateTime<Utc>,
    /// Computed aggregate value.
    pub value: f64,
    /// Number of raw data points that contributed to the aggregate.
    pub sample_count: u32,
}

/// Divide a slice of **sorted** data points into fixed-width windows and
/// compute the requested aggregation for each window that contains data.
pub fn aggregate_windows(
    points: &[DataPoint],
    range_start: DateTime<Utc>,
    range_end: DateTime<Utc>,
    window_secs: i64,
    aggregation: Aggregation,
) -> Vec<AggregatedPoint> {
    if points.is_empty() || window_secs <= 0 {
        return Vec::new();
    }

    let window_duration = Duration::seconds(window_secs);
    let mut results = Vec::new();
    let mut window_start = range_start;

    while window_start < range_end {
        let window_end = std::cmp::min(window_start + window_duration, range_end);

        let window_points: Vec<&DataPoint> = points
            .iter()
            .filter(|p| p.timestamp >= window_start && p.timestamp < window_end)
            .collect();

        if !window_points.is_empty() {
            let sample_count = window_points.len() as u32;
            let value = compute_aggregation(&window_points, aggregation);

            results.push(AggregatedPoint {
                window_start,
                window_end,
                value,
                sample_count,
            });
        }

        window_start = window_end;
    }

    results
}

/// Compute a single aggregation value over a non-empty slice of data points.
fn compute_aggregation(points: &[&DataPoint], aggregation: Aggregation) -> f64 {
    match aggregation {
        Aggregation::Avg => {
            let sum: f64 = points.iter().map(|p| p.value).sum();
            sum / points.len() as f64
        }
        Aggregation::Min => points.iter().map(|p| p.value).fold(f64::INFINITY, f64::min),
        Aggregation::Max => points
            .iter()
            .map(|p| p.value)
            .fold(f64::NEG_INFINITY, f64::max),
        Aggregation::Sum => points.iter().map(|p| p.value).sum(),
        Aggregation::Count => points.len() as f64,
        Aggregation::First => points.first().map_or(0.0, |p| p.value),
        Aggregation::Last => points.last().map_or(0.0, |p| p.value),
    }
}
