// ABOUTME: TimeSeriesStore trait defining the abstract storage interface for time-series data
// ABOUTME: Includes InMemoryStore implementation backed by Arc<RwLock<HashMap>> for tests and development
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::aggregation::{aggregate_windows, AggregatedPoint, Aggregation};
use crate::error::Result;
use crate::point::DataPoint;
use crate::query::{QueryResult, TimeRange};
use crate::series::TimeSeries;

/// Composite key for looking up a time series: (`source_id`, `series_type`).
type SeriesKey = (String, u32);

/// Abstract storage interface for time-series data.
///
/// Implementations may be backed by an in-memory `HashMap`, Postgres, or any
/// other persistent store. All methods are async to accommodate I/O-bound
/// backends.
#[async_trait]
pub trait TimeSeriesStore: Send + Sync {
    /// Insert a single data point into the specified series.
    async fn insert(&self, source_id: &str, series_type: u32, point: DataPoint) -> Result<()>;

    /// Insert multiple data points into the specified series.
    async fn insert_batch(
        &self,
        source_id: &str,
        series_type: u32,
        points: Vec<DataPoint>,
    ) -> Result<()>;

    /// Query data points within a time range.
    async fn query(
        &self,
        source_id: &str,
        series_type: u32,
        range: &TimeRange,
    ) -> Result<QueryResult>;

    /// Aggregate data points within a time range using fixed-width windows.
    async fn aggregate(
        &self,
        source_id: &str,
        series_type: u32,
        range: &TimeRange,
        window_secs: i64,
        aggregation: Aggregation,
    ) -> Result<Vec<AggregatedPoint>>;

    /// Return the most recent data point for the specified series.
    async fn latest(&self, source_id: &str, series_type: u32) -> Result<Option<DataPoint>>;

    /// Delete all data points within a time range and return the count removed.
    async fn delete_range(
        &self,
        source_id: &str,
        series_type: u32,
        range: &TimeRange,
    ) -> Result<u64>;
}

/// In-memory implementation of [`TimeSeriesStore`] for tests and development.
///
/// Data is stored in a `HashMap` keyed by `(source_id, series_type)` and
/// protected by a tokio `RwLock` for safe concurrent access.
#[derive(Debug, Clone)]
pub struct InMemoryStore {
    data: Arc<RwLock<HashMap<SeriesKey, TimeSeries>>>,
}

impl InMemoryStore {
    /// Create a new empty in-memory store.
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TimeSeriesStore for InMemoryStore {
    async fn insert(&self, source_id: &str, series_type: u32, point: DataPoint) -> Result<()> {
        let key = (source_id.to_owned(), series_type);
        let mut data = self.data.write().await;
        data.entry(key).or_default().insert(point);
        Ok(())
    }

    async fn insert_batch(
        &self,
        source_id: &str,
        series_type: u32,
        points: Vec<DataPoint>,
    ) -> Result<()> {
        let key = (source_id.to_owned(), series_type);
        let mut data = self.data.write().await;
        data.entry(key).or_default().insert_batch(points);
        Ok(())
    }

    async fn query(
        &self,
        source_id: &str,
        series_type: u32,
        range: &TimeRange,
    ) -> Result<QueryResult> {
        let key = (source_id.to_owned(), series_type);
        let data = self.data.read().await;
        let points = data.get(&key).map(|ts| ts.range(range)).unwrap_or_default();
        Ok(QueryResult::from_points(points))
    }

    async fn aggregate(
        &self,
        source_id: &str,
        series_type: u32,
        range: &TimeRange,
        window_secs: i64,
        aggregation: Aggregation,
    ) -> Result<Vec<AggregatedPoint>> {
        let key = (source_id.to_owned(), series_type);
        let data = self.data.read().await;
        let points = data.get(&key).map(|ts| ts.range(range)).unwrap_or_default();
        Ok(aggregate_windows(
            &points,
            range.start,
            range.end,
            window_secs,
            aggregation,
        ))
    }

    async fn latest(&self, source_id: &str, series_type: u32) -> Result<Option<DataPoint>> {
        let key = (source_id.to_owned(), series_type);
        let data = self.data.read().await;
        Ok(data.get(&key).and_then(|ts| ts.latest().cloned()))
    }

    async fn delete_range(
        &self,
        source_id: &str,
        series_type: u32,
        range: &TimeRange,
    ) -> Result<u64> {
        let key = (source_id.to_owned(), series_type);
        let mut data = self.data.write().await;
        let removed = data.get_mut(&key).map_or(0, |ts| ts.remove_range(range));
        Ok(removed)
    }
}
