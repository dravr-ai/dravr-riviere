# Changelog

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
