# Riviere -- Time-Series Storage Engine

[![CI](https://github.com/dravr-ai/dravr-riviere/actions/workflows/ci.yml/badge.svg)](https://github.com/dravr-ai/dravr-riviere/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Postgres-backed time-series storage engine for health and fitness metrics. Provides a trait-based storage interface with windowed aggregation, range queries, and a catalog of 90+ metric types following Open Wearables ID conventions.

## Table of Contents

- [Quick Start](#quick-start)
- [Core Types](#core-types)
- [Series Type Catalog](#series-type-catalog)
- [REST API Server](#rest-api-server-dravr-riviere-server)
- [MCP Server](#mcp-server-dravr-riviere-mcp)
- [Architecture](#architecture)
- [License](#license)

## Quick Start

### Library (Rust)

```toml
[dependencies]
dravr-riviere = "0.1"
```

```rust
use dravr_riviere::{DataPoint, InMemoryStore, TimeSeriesStore, TimeRange, Aggregation, SeriesType};
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() {
    let store = InMemoryStore::new();
    let now = Utc::now();

    // Insert heart rate data points
    let hr_id = SeriesType::HeartRate.id();
    store.insert("user-1", hr_id, DataPoint::new(now, 72.0)).await.unwrap();
    store.insert("user-1", hr_id, DataPoint::new(now + Duration::seconds(60), 75.0)).await.unwrap();

    // Query a time range
    let range = TimeRange::new(now - Duration::hours(1), now + Duration::hours(1));
    let result = store.query("user-1", hr_id, &range).await.unwrap();
    println!("Found {} data points", result.total_count);

    // Aggregate into 1-hour windows
    let agg = store.aggregate("user-1", hr_id, &range, 3600, Aggregation::Avg).await.unwrap();
    for window in &agg {
        println!("Window avg: {:.1} bpm ({} samples)", window.value, window.sample_count);
    }
}
```

### REST API Server

```bash
cargo run --bin dravr-riviere-server -- serve --port 3200
```

```bash
curl http://localhost:3200/health
# {"status":"ok","service":"dravr-riviere","version":"0.1.0"}
```

### MCP Server (stdio)

```bash
cargo run --bin dravr-riviere-mcp -- --transport stdio
```

Or over HTTP:

```bash
cargo run --bin dravr-riviere-mcp -- --transport http --port 3200
```

## Core Types

| Type | Description |
|------|-------------|
| `MetricKey` | Trait for generic metric identification (`as_str()` + `id()`) |
| `DataPoint` | Timestamped numeric measurement |
| `TimeSeries` | Sorted collection of `DataPoint` values with range queries |
| `TimeRange` | Half-open `[start, end)` query bounds |
| `TimeSeriesStore` | Async trait for pluggable storage backends |
| `InMemoryStore` | `HashMap`-backed implementation for tests/development |
| `Aggregation` | Windowed rollup functions (avg, min, max, sum, count, first, last) |
| `DataPointArchive` | Daily pre-aggregated data for long-term retention |

## Series Type Catalog

90+ health and fitness metric types organized by ID ranges:

| Range | Category | Examples |
|-------|----------|----------|
| 1-19 | Cardiovascular | heart_rate, resting_heart_rate, hrv_sdnn, hrv_rmssd |
| 20-39 | Blood & Respiratory | spo2, blood_glucose, blood_pressure_systolic, respiratory_rate |
| 40-59 | Body Composition | weight, body_fat_percentage, bmi, muscle_mass, skin_temperature |
| 60-79 | Fitness | vo2_max, ftp, critical_power, training_load |
| 80-99 | Activity Basic | steps, active_energy, exercise_time, floors_climbed |
| 100-119 | Distance | walking_running_distance, cycling_distance, elevation_gain |
| 120-139 | Walking Metrics | step_length, walking_speed, walking_asymmetry |
| 140-159 | Running Metrics | running_power, vertical_oscillation, ground_contact_time |
| 160-179 | Swimming | stroke_count, swolf, underwater_depth |
| 180-199 | Generic Activity | cadence, power, speed, tss, normalized_power |
| 200-219 | Environmental | audio_exposure, uv_index, ambient_temperature |
| 220-239 | Garmin-specific | stress_level, body_battery, training_readiness |

## Architecture

```
dravr-riviere/
├── src/                         # Core library (trait-based storage, zero I/O deps)
│   ├── lib.rs                   # Public API and module declarations
│   ├── error.rs                 # RiviereError structured error types
│   ├── key.rs                   # MetricKey trait
│   ├── point.rs                 # DataPoint (timestamp + value)
│   ├── series.rs                # TimeSeries sorted collection
│   ├── query.rs                 # TimeRange, QueryResult
│   ├── aggregation.rs           # Windowed aggregation (avg, min, max, sum, count, first, last)
│   ├── store.rs                 # TimeSeriesStore trait + InMemoryStore
│   ├── series_type.rs           # SeriesType enum (90+ metrics)
│   └── archive.rs               # DataPointArchive model
│
├── crates/
│   ├── dravr-riviere-mcp/       # MCP server (library + binary, powered by dravr-tronc)
│   │   ├── src/state.rs         # SharedState container
│   │   └── src/tools/           # MCP tool implementations
│   │
│   └── dravr-riviere-server/    # Unified REST API + MCP server (powered by dravr-tronc)
│       ├── src/router.rs        # Axum routes (/health, /mcp)
│       ├── src/auth.rs          # Bearer token middleware
│       └── src/main.rs          # CLI (serve, stdio)
│
└── tests/
    └── store_test.rs            # InMemoryStore integration tests
```

## Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `RIVIERE_API_TOKEN` | *(none)* | Bearer token for REST API auth (empty = no auth) |
| `RUST_LOG` | `info` | Log level |

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).
