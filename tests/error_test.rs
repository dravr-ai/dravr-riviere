// ABOUTME: Comprehensive tests for RiviereError structured error types
// ABOUTME: Validates Display messages, std::error::Error implementation, and variant construction
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use dravr_riviere::RiviereError;

// ── SeriesNotFound ───────────────────────────────────────────────────────

#[test]
fn series_not_found_display_contains_source_and_type() {
    let err = RiviereError::SeriesNotFound {
        source_id: "user-42".to_owned(),
        series_type: 1,
    };

    let msg = err.to_string();
    assert!(msg.contains("user-42"), "Display should contain source_id");
    assert!(msg.contains("type=1"), "Display should contain series_type");
}

#[test]
fn series_not_found_display_format() {
    let err = RiviereError::SeriesNotFound {
        source_id: "device-abc".to_owned(),
        series_type: 80,
    };

    let msg = err.to_string();
    assert_eq!(msg, "series not found: source=device-abc, type=80");
}

// ── InvalidQuery ─────────────────────────────────────────────────────────

#[test]
fn invalid_query_display_contains_reason() {
    let err = RiviereError::InvalidQuery {
        reason: "start exceeds end".to_owned(),
    };

    let msg = err.to_string();
    assert!(
        msg.contains("start exceeds end"),
        "Display should contain reason"
    );
}

#[test]
fn invalid_query_display_format() {
    let err = RiviereError::InvalidQuery {
        reason: "window must be positive".to_owned(),
    };

    assert_eq!(err.to_string(), "invalid query: window must be positive");
}

// ── InvalidDataPoint ─────────────────────────────────────────────────────

#[test]
fn invalid_data_point_display_contains_reason() {
    let err = RiviereError::InvalidDataPoint {
        reason: "value is NaN".to_owned(),
    };

    let msg = err.to_string();
    assert!(msg.contains("NaN"), "Display should contain reason");
}

#[test]
fn invalid_data_point_display_format() {
    let err = RiviereError::InvalidDataPoint {
        reason: "timestamp in future".to_owned(),
    };

    assert_eq!(err.to_string(), "invalid data point: timestamp in future");
}

// ── UnknownSeriesType ────────────────────────────────────────────────────

#[test]
fn unknown_series_type_display_contains_id() {
    let err = RiviereError::UnknownSeriesType { id: 9999 };

    let msg = err.to_string();
    assert!(
        msg.contains("9999"),
        "Display should contain the unknown id"
    );
}

#[test]
fn unknown_series_type_display_format() {
    let err = RiviereError::UnknownSeriesType { id: 42 };

    assert_eq!(err.to_string(), "unknown series type id: 42");
}

// ── Storage ──────────────────────────────────────────────────────────────

#[test]
fn storage_error_display_contains_message() {
    let err = RiviereError::Storage {
        message: "connection refused".to_owned(),
    };

    let msg = err.to_string();
    assert!(
        msg.contains("connection refused"),
        "Display should contain message",
    );
}

#[test]
fn storage_error_display_format() {
    let err = RiviereError::Storage {
        message: "disk full".to_owned(),
    };

    assert_eq!(err.to_string(), "storage error: disk full");
}

// ── std::error::Error Implementation ─────────────────────────────────────

#[test]
fn all_variants_implement_error_trait() {
    let errors: Vec<Box<dyn std::error::Error>> = vec![
        Box::new(RiviereError::SeriesNotFound {
            source_id: "x".to_owned(),
            series_type: 1,
        }),
        Box::new(RiviereError::InvalidQuery {
            reason: "r".to_owned(),
        }),
        Box::new(RiviereError::InvalidDataPoint {
            reason: "r".to_owned(),
        }),
        Box::new(RiviereError::UnknownSeriesType { id: 0 }),
        Box::new(RiviereError::Storage {
            message: "m".to_owned(),
        }),
    ];

    for err in &errors {
        // Verify Display works (via Error trait)
        let display = format!("{err}");
        assert!(!display.is_empty(), "Error display should not be empty");
    }
}

#[test]
fn all_variants_implement_display() {
    let errors: Vec<RiviereError> = vec![
        RiviereError::SeriesNotFound {
            source_id: "s".to_owned(),
            series_type: 1,
        },
        RiviereError::InvalidQuery {
            reason: "q".to_owned(),
        },
        RiviereError::InvalidDataPoint {
            reason: "d".to_owned(),
        },
        RiviereError::UnknownSeriesType { id: 0 },
        RiviereError::Storage {
            message: "m".to_owned(),
        },
    ];

    for err in &errors {
        let display = format!("{err}");
        assert!(
            !display.is_empty(),
            "Display should not be empty for {err:?}"
        );
    }
}

// ── Debug ────────────────────────────────────────────────────────────────

#[test]
fn all_variants_implement_debug() {
    let errors: Vec<RiviereError> = vec![
        RiviereError::SeriesNotFound {
            source_id: "s".to_owned(),
            series_type: 1,
        },
        RiviereError::InvalidQuery {
            reason: "q".to_owned(),
        },
        RiviereError::InvalidDataPoint {
            reason: "d".to_owned(),
        },
        RiviereError::UnknownSeriesType { id: 0 },
        RiviereError::Storage {
            message: "m".to_owned(),
        },
    ];

    for err in &errors {
        let debug = format!("{err:?}");
        assert!(!debug.is_empty(), "Debug should not be empty");
    }
}

// ── Result Type Alias ────────────────────────────────────────────────────

#[test]
fn result_type_alias_accepts_ok_variant() {
    // Verify the Result alias accepts Ok values via pattern match
    let result: dravr_riviere::Result<i32> = std::hint::black_box(Ok(42));
    match result {
        Ok(val) => assert_eq!(val, 42),
        Err(e) => panic!("Expected Ok, got Err: {e}"),
    }
}

#[test]
fn result_type_alias_accepts_err_variant() {
    let result: dravr_riviere::Result<i32> =
        std::hint::black_box(Err(RiviereError::UnknownSeriesType { id: 0 }));
    match result {
        Ok(val) => panic!("Expected Err, got Ok({val})"),
        Err(e) => assert!(e.to_string().contains('0')),
    }
}
