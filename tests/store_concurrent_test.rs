// ABOUTME: Concurrency tests for the InMemoryStore async time-series store implementation
// ABOUTME: Validates thread safety, concurrent reads/writes, and Send+Sync guarantees
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use std::sync::Arc;

use chrono::{Duration, Utc};

use dravr_riviere::point::DataPoint;
use dravr_riviere::query::TimeRange;
use dravr_riviere::store::{InMemoryStore, TimeSeriesStore};

const SERIES_TYPE: u32 = 1;

// ── Send + Sync Compile-Time Check ───────────────────────────────────────

#[test]
fn store_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<InMemoryStore>();
}

#[test]
fn store_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<InMemoryStore>();
}

// ── Concurrent Inserts to Same Series ────────────────────────────────────

#[tokio::test]
async fn concurrent_inserts_to_same_source_and_type() {
    let store = Arc::new(InMemoryStore::new());
    let base = Utc::now();
    let task_count: i32 = 10;
    let points_per_task: i32 = 100;

    let mut handles = Vec::new();
    for task_idx in 0..task_count {
        let store_ref = Arc::clone(&store);
        let handle = tokio::spawn(async move {
            for i in 0..points_per_task {
                let offset = task_idx * points_per_task + i;
                let point = DataPoint::new(
                    base + Duration::milliseconds(i64::from(offset)),
                    f64::from(offset),
                );
                store_ref
                    .insert("user-1", SERIES_TYPE, point)
                    .await
                    .expect("concurrent insert should succeed");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("task should complete without panic");
    }

    // All points should be present
    let range = TimeRange::new(
        base - Duration::seconds(1),
        base + Duration::seconds(i64::from(task_count) * i64::from(points_per_task)),
    );
    let result = store
        .query("user-1", SERIES_TYPE, &range)
        .await
        .expect("query should succeed after concurrent inserts");

    assert_eq!(
        result.total_count,
        (task_count * points_per_task) as usize,
        "All concurrently inserted points must be present",
    );
}

// ── Concurrent Insert and Query ──────────────────────────────────────────

#[tokio::test]
async fn concurrent_insert_and_query() {
    let store = Arc::new(InMemoryStore::new());
    let base = Utc::now();

    // Pre-populate with some data
    for i in 0i32..50 {
        store
            .insert(
                "user-1",
                SERIES_TYPE,
                DataPoint::new(base + Duration::seconds(i64::from(i)), f64::from(i)),
            )
            .await
            .expect("pre-populate insert should succeed");
    }

    let writer_store = Arc::clone(&store);
    let reader_store = Arc::clone(&store);

    // Writer task: insert more points
    let writer = tokio::spawn(async move {
        for i in 50i32..150 {
            writer_store
                .insert(
                    "user-1",
                    SERIES_TYPE,
                    DataPoint::new(base + Duration::seconds(i64::from(i)), f64::from(i)),
                )
                .await
                .expect("writer insert should succeed");
        }
    });

    // Reader task: repeatedly query
    let reader = tokio::spawn(async move {
        let range = TimeRange::new(base - Duration::seconds(1), base + Duration::seconds(200));
        for _ in 0..50 {
            let result = reader_store
                .query("user-1", SERIES_TYPE, &range)
                .await
                .expect("concurrent read should succeed");
            // Count should be somewhere between 50 (pre-populated) and 150 (all inserted)
            assert!(
                result.total_count >= 50,
                "Should have at least the pre-populated points",
            );
        }
    });

    writer.await.expect("writer task should complete");
    reader.await.expect("reader task should complete");
}

// ── Concurrent Insert and Delete ─────────────────────────────────────────

#[tokio::test]
async fn concurrent_insert_and_delete_range() {
    let store = Arc::new(InMemoryStore::new());
    let base = Utc::now();

    // Pre-populate
    for i in 0i32..100 {
        store
            .insert(
                "user-1",
                SERIES_TYPE,
                DataPoint::new(base + Duration::seconds(i64::from(i)), f64::from(i)),
            )
            .await
            .expect("pre-populate insert should succeed");
    }

    let writer_store = Arc::clone(&store);
    let deleter_store = Arc::clone(&store);

    // Writer inserts at offsets 200+
    let writer = tokio::spawn(async move {
        for i in 200i32..300 {
            writer_store
                .insert(
                    "user-1",
                    SERIES_TYPE,
                    DataPoint::new(base + Duration::seconds(i64::from(i)), f64::from(i)),
                )
                .await
                .expect("writer insert should succeed");
        }
    });

    // Deleter removes points in the 0-50 range
    let deleter = tokio::spawn(async move {
        let delete_range = TimeRange::new(base, base + Duration::seconds(50));
        let removed = deleter_store
            .delete_range("user-1", SERIES_TYPE, &delete_range)
            .await
            .expect("delete_range should succeed");
        // Should remove at least some points (0 through 49)
        assert!(removed > 0, "Should have removed some points");
    });

    writer.await.expect("writer task should complete");
    deleter.await.expect("deleter task should complete");

    // Verify the store is in a consistent state
    let full_range = TimeRange::new(base - Duration::seconds(1), base + Duration::seconds(500));
    let result = store
        .query("user-1", SERIES_TYPE, &full_range)
        .await
        .expect("query should succeed");

    // We started with 100, deleted some from [0,50), and added 100 more at [200,300)
    // The remaining should be points from [50,100) + [200,300)
    assert!(result.total_count > 0, "Store should have remaining points");
}

// ── Multiple Concurrent Readers with One Writer ──────────────────────────

#[tokio::test]
async fn multiple_readers_concurrent_with_one_writer() {
    let store = Arc::new(InMemoryStore::new());
    let base = Utc::now();

    // Pre-populate
    for i in 0i32..20 {
        store
            .insert(
                "user-1",
                SERIES_TYPE,
                DataPoint::new(base + Duration::seconds(i64::from(i)), f64::from(i)),
            )
            .await
            .expect("pre-populate insert should succeed");
    }

    let writer_store = Arc::clone(&store);

    let writer = tokio::spawn(async move {
        for i in 20i32..120 {
            writer_store
                .insert(
                    "user-1",
                    SERIES_TYPE,
                    DataPoint::new(base + Duration::seconds(i64::from(i)), f64::from(i)),
                )
                .await
                .expect("writer insert should succeed");
        }
    });

    let reader_count = 5;
    let mut reader_handles = Vec::new();
    for _ in 0..reader_count {
        let reader_store = Arc::clone(&store);
        let handle = tokio::spawn(async move {
            let range = TimeRange::new(base - Duration::seconds(1), base + Duration::seconds(200));
            for _ in 0..20 {
                let result = reader_store
                    .query("user-1", SERIES_TYPE, &range)
                    .await
                    .expect("reader query should succeed");
                assert!(
                    result.total_count >= 20,
                    "Should have at least pre-populated points",
                );
            }
        });
        reader_handles.push(handle);
    }

    writer.await.expect("writer task should complete");
    for handle in reader_handles {
        handle.await.expect("reader task should complete");
    }

    // Final verification: all 120 points present
    let range = TimeRange::new(base - Duration::seconds(1), base + Duration::seconds(200));
    let result = store
        .query("user-1", SERIES_TYPE, &range)
        .await
        .expect("final query should succeed");
    assert_eq!(result.total_count, 120);
}

// ── Concurrent Operations on Different Series ────────────────────────────

#[tokio::test]
async fn concurrent_operations_on_different_series() {
    let store = Arc::new(InMemoryStore::new());
    let base = Utc::now();

    let mut handles = Vec::new();

    // Each task operates on a different source_id
    for task_idx in 0u32..10 {
        let store_ref = Arc::clone(&store);
        let source_id = format!("user-{task_idx}");
        let handle = tokio::spawn(async move {
            for i in 0i32..50 {
                let point = DataPoint::new(
                    base + Duration::seconds(i64::from(i)),
                    f64::from(task_idx * 100 + i as u32),
                );
                store_ref
                    .insert(&source_id, SERIES_TYPE, point)
                    .await
                    .expect("insert to isolated series should succeed");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Verify each source has exactly 50 points
    let range = TimeRange::new(base - Duration::seconds(1), base + Duration::seconds(100));
    for task_idx in 0u32..10 {
        let source_id = format!("user-{task_idx}");
        let result = store
            .query(&source_id, SERIES_TYPE, &range)
            .await
            .expect("query should succeed");
        assert_eq!(
            result.total_count, 50,
            "source {source_id} should have 50 points",
        );
    }
}

// ── Concurrent Latest Reads ──────────────────────────────────────────────

#[tokio::test]
async fn concurrent_latest_reads_with_inserts() {
    let store = Arc::new(InMemoryStore::new());
    let base = Utc::now();

    let writer_store = Arc::clone(&store);
    let reader_store = Arc::clone(&store);

    let writer = tokio::spawn(async move {
        for i in 0i32..100 {
            writer_store
                .insert(
                    "user-1",
                    SERIES_TYPE,
                    DataPoint::new(base + Duration::seconds(i64::from(i)), f64::from(i)),
                )
                .await
                .expect("insert should succeed");
        }
    });

    let reader = tokio::spawn(async move {
        for _ in 0..50 {
            // latest() should never fail, even if the series is being modified
            let _ = reader_store.latest("user-1", SERIES_TYPE).await;
        }
    });

    writer.await.expect("writer should complete");
    reader.await.expect("reader should complete");

    // After all writes, latest should be the point at offset 99
    let latest = store
        .latest("user-1", SERIES_TYPE)
        .await
        .expect("latest query should succeed")
        .expect("should have a latest point after 100 inserts");

    assert!((latest.value - 99.0).abs() < f64::EPSILON);
}
