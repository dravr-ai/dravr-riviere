# Changelog

## [0.2.0] — 2026-03-25



## [0.1.0] — 2026-03-25

### Added

- feat: initial scaffold — time-series storage engine with trait-based store 3 workspace crates (core, MCP, server), 107 SeriesType variants, 24 tests



## [0.1.0] - 2026-03-24

### Added
- `MetricKey` trait for generic metric identification
- `DataPoint` timestamped value type
- `TimeSeries` sorted collection with range queries
- `TimeSeriesStore` trait for pluggable storage backends
- `InMemoryStore` implementation for testing and development
- `SeriesType` enum with 90+ health and fitness metric types
- Windowed aggregation (avg, min, max, sum, count, first, last)
- `DataPointArchive` model for daily aggregation lifecycle
- MCP server via dravr-tronc
- REST API + MCP unified server
