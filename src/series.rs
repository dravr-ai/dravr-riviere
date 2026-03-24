// ABOUTME: TimeSeries struct holding a sorted collection of DataPoints for a single metric
// ABOUTME: Provides insertion with automatic sort maintenance and range-based querying
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use crate::point::DataPoint;
use crate::query::TimeRange;

/// A sorted collection of [`DataPoint`] values for a single metric series.
///
/// Points are kept in ascending timestamp order. Duplicate timestamps are
/// permitted (the second insert lands immediately after the first).
#[derive(Debug, Clone, Default)]
pub struct TimeSeries {
    points: Vec<DataPoint>,
}

impl TimeSeries {
    /// Create an empty time series.
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Create a time series with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            points: Vec::with_capacity(capacity),
        }
    }

    /// Insert a single data point, maintaining sorted order.
    pub fn insert(&mut self, point: DataPoint) {
        let pos = self
            .points
            .binary_search_by(|p| p.timestamp.cmp(&point.timestamp))
            .unwrap_or_else(|i| i);
        self.points.insert(pos, point);
    }

    /// Insert multiple data points, maintaining sorted order.
    pub fn insert_batch(&mut self, points: Vec<DataPoint>) {
        self.points.reserve(points.len());
        for point in points {
            self.insert(point);
        }
    }

    /// Return all points whose timestamp falls within the given range.
    pub fn range(&self, range: &TimeRange) -> Vec<DataPoint> {
        self.points
            .iter()
            .filter(|p| range.contains(p.timestamp))
            .cloned()
            .collect()
    }

    /// Return the most recent data point, if any.
    pub fn latest(&self) -> Option<&DataPoint> {
        self.points.last()
    }

    /// Remove all points whose timestamp falls within the given range.
    ///
    /// Returns the number of points removed.
    pub fn remove_range(&mut self, range: &TimeRange) -> u64 {
        let before = self.points.len();
        self.points.retain(|p| !range.contains(p.timestamp));
        (before - self.points.len()) as u64
    }

    /// Total number of data points in this series.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Whether the series contains no data points.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Borrow the underlying sorted slice.
    pub fn points(&self) -> &[DataPoint] {
        &self.points
    }
}
