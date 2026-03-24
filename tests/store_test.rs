// ABOUTME: Comprehensive integration tests for the InMemoryStore implementation
// ABOUTME: Validates insert, query, aggregate, latest, delete, and batch operations
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use chrono::{Duration, Utc};

use dravr_riviere::aggregation::Aggregation;
use dravr_riviere::point::DataPoint;
use dravr_riviere::query::TimeRange;
use dravr_riviere::series_type::SeriesType;
use dravr_riviere::store::{InMemoryStore, TimeSeriesStore};

fn heart_rate_id() -> u32 {
    SeriesType::HeartRate.id()
}

fn make_store() -> InMemoryStore {
    InMemoryStore::new()
}

fn now_plus(secs: i64) -> chrono::DateTime<Utc> {
    Utc::now() + Duration::seconds(secs)
}

// ── Insert & Query ────────────────────────────────────────────────────────

#[tokio::test]
async fn insert_and_query_single_point() {
    let store = make_store();
    let ts = Utc::now();
    let point = DataPoint::new(ts, 72.0);

    store
        .insert("user-1", heart_rate_id(), point.clone())
        .await
        .unwrap();

    let range = TimeRange::new(ts - Duration::seconds(1), ts + Duration::seconds(1));
    let result = store
        .query("user-1", heart_rate_id(), &range)
        .await
        .unwrap();

    assert_eq!(result.points.len(), 1);
    assert_eq!(result.total_count, 1);
    assert!((result.points[0].value - 72.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn query_empty_store_returns_empty() {
    let store = make_store();
    let range = TimeRange::new(Utc::now() - Duration::hours(1), Utc::now());
    let result = store
        .query("user-1", heart_rate_id(), &range)
        .await
        .unwrap();

    assert!(result.points.is_empty());
    assert_eq!(result.total_count, 0);
}

#[tokio::test]
async fn query_respects_time_range_bounds() {
    let store = make_store();
    let base = Utc::now();

    for i in 0i32..10 {
        let point = DataPoint::new(base + Duration::seconds(i64::from(i) * 10), f64::from(i));
        store
            .insert("user-1", heart_rate_id(), point)
            .await
            .unwrap();
    }

    // Query range that captures points at offsets 20, 30, 40 (indices 2,3,4)
    let range = TimeRange::new(base + Duration::seconds(20), base + Duration::seconds(50));
    let result = store
        .query("user-1", heart_rate_id(), &range)
        .await
        .unwrap();

    assert_eq!(result.points.len(), 3);
    assert!((result.points[0].value - 2.0).abs() < f64::EPSILON);
    assert!((result.points[2].value - 4.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn different_sources_are_isolated() {
    let store = make_store();
    let ts = Utc::now();

    store
        .insert("user-1", heart_rate_id(), DataPoint::new(ts, 70.0))
        .await
        .unwrap();
    store
        .insert("user-2", heart_rate_id(), DataPoint::new(ts, 80.0))
        .await
        .unwrap();

    let range = TimeRange::new(ts - Duration::seconds(1), ts + Duration::seconds(1));

    let r1 = store
        .query("user-1", heart_rate_id(), &range)
        .await
        .unwrap();
    let r2 = store
        .query("user-2", heart_rate_id(), &range)
        .await
        .unwrap();

    assert_eq!(r1.points.len(), 1);
    assert!((r1.points[0].value - 70.0).abs() < f64::EPSILON);
    assert_eq!(r2.points.len(), 1);
    assert!((r2.points[0].value - 80.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn different_series_types_are_isolated() {
    let store = make_store();
    let ts = Utc::now();
    let steps_id = SeriesType::Steps.id();

    store
        .insert("user-1", heart_rate_id(), DataPoint::new(ts, 72.0))
        .await
        .unwrap();
    store
        .insert("user-1", steps_id, DataPoint::new(ts, 8500.0))
        .await
        .unwrap();

    let range = TimeRange::new(ts - Duration::seconds(1), ts + Duration::seconds(1));

    let hr = store
        .query("user-1", heart_rate_id(), &range)
        .await
        .unwrap();
    let steps = store.query("user-1", steps_id, &range).await.unwrap();

    assert_eq!(hr.points.len(), 1);
    assert!((hr.points[0].value - 72.0).abs() < f64::EPSILON);
    assert_eq!(steps.points.len(), 1);
    assert!((steps.points[0].value - 8500.0).abs() < f64::EPSILON);
}

// ── Batch Insert ──────────────────────────────────────────────────────────

#[tokio::test]
async fn insert_batch_stores_all_points_sorted() {
    let store = make_store();
    let base = Utc::now();

    let points = vec![
        DataPoint::new(base + Duration::seconds(30), 3.0),
        DataPoint::new(base + Duration::seconds(10), 1.0),
        DataPoint::new(base + Duration::seconds(20), 2.0),
    ];

    store
        .insert_batch("user-1", heart_rate_id(), points)
        .await
        .unwrap();

    let range = TimeRange::new(base, base + Duration::seconds(60));
    let result = store
        .query("user-1", heart_rate_id(), &range)
        .await
        .unwrap();

    assert_eq!(result.points.len(), 3);
    assert!((result.points[0].value - 1.0).abs() < f64::EPSILON);
    assert!((result.points[1].value - 2.0).abs() < f64::EPSILON);
    assert!((result.points[2].value - 3.0).abs() < f64::EPSILON);
}

// ── Latest ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn latest_returns_most_recent_point() {
    let store = make_store();
    let base = Utc::now();

    for i in 0i32..5 {
        store
            .insert(
                "user-1",
                heart_rate_id(),
                DataPoint::new(
                    base + Duration::seconds(i64::from(i) * 10),
                    60.0 + f64::from(i),
                ),
            )
            .await
            .unwrap();
    }

    let latest = store
        .latest("user-1", heart_rate_id())
        .await
        .unwrap()
        .unwrap();
    assert!((latest.value - 64.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn latest_returns_none_for_empty_series() {
    let store = make_store();
    let result = store.latest("user-1", heart_rate_id()).await.unwrap();
    assert!(result.is_none());
}

// ── Delete Range ──────────────────────────────────────────────────────────

#[tokio::test]
async fn delete_range_removes_matching_points() {
    let store = make_store();
    let base = Utc::now();

    for i in 0i32..10 {
        store
            .insert(
                "user-1",
                heart_rate_id(),
                DataPoint::new(base + Duration::seconds(i64::from(i) * 10), f64::from(i)),
            )
            .await
            .unwrap();
    }

    // Delete points at offsets 20, 30, 40 (indices 2,3,4)
    let delete_range = TimeRange::new(base + Duration::seconds(20), base + Duration::seconds(50));
    let removed = store
        .delete_range("user-1", heart_rate_id(), &delete_range)
        .await
        .unwrap();

    assert_eq!(removed, 3);

    let full_range = TimeRange::new(base, base + Duration::seconds(100));
    let remaining = store
        .query("user-1", heart_rate_id(), &full_range)
        .await
        .unwrap();
    assert_eq!(remaining.points.len(), 7);
}

#[tokio::test]
async fn delete_range_on_missing_series_returns_zero() {
    let store = make_store();
    let range = TimeRange::new(Utc::now() - Duration::hours(1), Utc::now());
    let removed = store
        .delete_range("nonexistent", heart_rate_id(), &range)
        .await
        .unwrap();
    assert_eq!(removed, 0);
}

// ── Aggregation ───────────────────────────────────────────────────────────

#[tokio::test]
async fn aggregate_avg_over_single_window() {
    let store = make_store();
    let base = Utc::now();

    let points = vec![
        DataPoint::new(base + Duration::seconds(1), 60.0),
        DataPoint::new(base + Duration::seconds(2), 70.0),
        DataPoint::new(base + Duration::seconds(3), 80.0),
    ];

    store
        .insert_batch("user-1", heart_rate_id(), points)
        .await
        .unwrap();

    let range = TimeRange::new(base, base + Duration::seconds(10));
    let agg = store
        .aggregate("user-1", heart_rate_id(), &range, 10, Aggregation::Avg)
        .await
        .unwrap();

    assert_eq!(agg.len(), 1);
    assert!((agg[0].value - 70.0).abs() < f64::EPSILON);
    assert_eq!(agg[0].sample_count, 3);
}

#[tokio::test]
async fn aggregate_min_max_sum_count() {
    let store = make_store();
    let base = Utc::now();

    let points = vec![
        DataPoint::new(base + Duration::seconds(1), 10.0),
        DataPoint::new(base + Duration::seconds(2), 30.0),
        DataPoint::new(base + Duration::seconds(3), 20.0),
    ];

    store
        .insert_batch("user-1", heart_rate_id(), points)
        .await
        .unwrap();

    let range = TimeRange::new(base, base + Duration::seconds(10));

    let min = store
        .aggregate("user-1", heart_rate_id(), &range, 10, Aggregation::Min)
        .await
        .unwrap();
    assert!((min[0].value - 10.0).abs() < f64::EPSILON);

    let max = store
        .aggregate("user-1", heart_rate_id(), &range, 10, Aggregation::Max)
        .await
        .unwrap();
    assert!((max[0].value - 30.0).abs() < f64::EPSILON);

    let sum = store
        .aggregate("user-1", heart_rate_id(), &range, 10, Aggregation::Sum)
        .await
        .unwrap();
    assert!((sum[0].value - 60.0).abs() < f64::EPSILON);

    let count = store
        .aggregate("user-1", heart_rate_id(), &range, 10, Aggregation::Count)
        .await
        .unwrap();
    assert!((count[0].value - 3.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn aggregate_first_last() {
    let store = make_store();
    let base = Utc::now();

    let points = vec![
        DataPoint::new(base + Duration::seconds(1), 100.0),
        DataPoint::new(base + Duration::seconds(2), 200.0),
        DataPoint::new(base + Duration::seconds(3), 300.0),
    ];

    store
        .insert_batch("user-1", heart_rate_id(), points)
        .await
        .unwrap();

    let range = TimeRange::new(base, base + Duration::seconds(10));

    let first = store
        .aggregate("user-1", heart_rate_id(), &range, 10, Aggregation::First)
        .await
        .unwrap();
    assert!((first[0].value - 100.0).abs() < f64::EPSILON);

    let last = store
        .aggregate("user-1", heart_rate_id(), &range, 10, Aggregation::Last)
        .await
        .unwrap();
    assert!((last[0].value - 300.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn aggregate_multiple_windows() {
    let store = make_store();
    let base = Utc::now();

    // 6 points over 60 seconds
    for i in 0i32..6 {
        store
            .insert(
                "user-1",
                heart_rate_id(),
                DataPoint::new(
                    base + Duration::seconds(i64::from(i) * 10 + 1),
                    f64::from(i + 1) * 10.0,
                ),
            )
            .await
            .unwrap();
    }

    let range = TimeRange::new(base, base + Duration::seconds(60));
    let agg = store
        .aggregate("user-1", heart_rate_id(), &range, 30, Aggregation::Avg)
        .await
        .unwrap();

    assert_eq!(agg.len(), 2);
    // Window 1 (0-30s): points at 1s=10, 11s=20, 21s=30 => avg=20
    assert!((agg[0].value - 20.0).abs() < f64::EPSILON);
    assert_eq!(agg[0].sample_count, 3);
    // Window 2 (30-60s): points at 31s=40, 41s=50, 51s=60 => avg=50
    assert!((agg[1].value - 50.0).abs() < f64::EPSILON);
    assert_eq!(agg[1].sample_count, 3);
}

#[tokio::test]
async fn aggregate_empty_series_returns_empty() {
    let store = make_store();
    let range = TimeRange::new(Utc::now() - Duration::hours(1), Utc::now());
    let agg = store
        .aggregate("user-1", heart_rate_id(), &range, 3600, Aggregation::Avg)
        .await
        .unwrap();
    assert!(agg.is_empty());
}

// ── SeriesType Catalog ────────────────────────────────────────────────────

#[test]
fn series_type_round_trip_by_id() {
    let st = SeriesType::HeartRate;
    assert_eq!(st.id(), 1);
    assert_eq!(SeriesType::from_id(1), Some(SeriesType::HeartRate));

    let st2 = SeriesType::Steps;
    assert_eq!(st2.id(), 80);
    assert_eq!(SeriesType::from_id(80), Some(SeriesType::Steps));
}

#[test]
fn series_type_from_id_returns_none_for_unknown() {
    assert!(SeriesType::from_id(9999).is_none());
}

#[test]
fn series_type_as_str_is_snake_case() {
    assert_eq!(SeriesType::HeartRate.as_str(), "heart_rate");
    assert_eq!(SeriesType::Vo2Max.as_str(), "vo2_max");
    assert_eq!(SeriesType::BodyBattery.as_str(), "body_battery");
}

#[test]
fn all_series_types_has_more_than_90_entries() {
    assert!(
        dravr_riviere::ALL_SERIES_TYPES.len() >= 90,
        "Expected 90+ series types, got {}",
        dravr_riviere::ALL_SERIES_TYPES.len()
    );
}

#[test]
fn all_series_types_have_unique_ids() {
    let mut ids: Vec<u32> = dravr_riviere::ALL_SERIES_TYPES
        .iter()
        .map(|st| st.id())
        .collect();
    let before = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), before, "Duplicate series type IDs found");
}

// ── DataPoint Ordering ────────────────────────────────────────────────────

#[test]
fn data_points_sort_by_timestamp() {
    let t1 = now_plus(0);
    let t2 = now_plus(10);
    let t3 = now_plus(20);

    let mut points = [
        DataPoint::new(t3, 3.0),
        DataPoint::new(t1, 1.0),
        DataPoint::new(t2, 2.0),
    ];

    points.sort();

    assert!((points[0].value - 1.0).abs() < f64::EPSILON);
    assert!((points[1].value - 2.0).abs() < f64::EPSILON);
    assert!((points[2].value - 3.0).abs() < f64::EPSILON);
}

// ── TimeRange ─────────────────────────────────────────────────────────────

#[test]
fn time_range_contains_checks_half_open_interval() {
    let start = Utc::now();
    let end = start + Duration::seconds(60);
    let range = TimeRange::new(start, end);

    assert!(range.contains(start));
    assert!(range.contains(start + Duration::seconds(30)));
    assert!(!range.contains(end)); // exclusive upper bound
    assert!(!range.contains(start - Duration::seconds(1)));
}

#[test]
fn time_range_duration_secs() {
    let start = Utc::now();
    let end = start + Duration::seconds(3600);
    let range = TimeRange::new(start, end);
    assert_eq!(range.duration_secs(), 3600);
}

// ── DataPointArchive ──────────────────────────────────────────────────────

#[test]
fn archive_construction() {
    let archive = dravr_riviere::DataPointArchive::new(
        "user-1".to_owned(),
        heart_rate_id(),
        Utc::now(),
        Aggregation::Avg,
        72.5,
        1440,
    );

    assert_eq!(archive.source_id, "user-1");
    assert_eq!(archive.series_type_id, heart_rate_id());
    assert!((archive.value - 72.5).abs() < f64::EPSILON);
    assert_eq!(archive.sample_count, 1440);
}
