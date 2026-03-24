## Git Workflow: NO Pull Requests

**CRITICAL: NEVER create Pull Requests. All merges happen locally via squash merge.**

### Rules
- **NEVER use `gh pr create`** or any PR creation command
- **NEVER suggest creating a PR**
- Feature branches are merged via **local squash merge**

### Workflow for Features
1. Create feature branch: `git checkout -b feature/my-feature`
2. Make commits, push to remote: `git push -u origin feature/my-feature`
3. When ready, squash merge locally (from main worktree):
   ```bash
   git checkout main
   git fetch origin
   git merge --squash origin/feature/my-feature
   git commit
   git push
   ```

### Bug Fixes
- Bug fixes go directly to `main` branch (no feature branch needed)
- Commit and push directly: `git push origin main`

## Project Overview

**dravr-riviere** is a Rust workspace providing a Postgres-backed time-series storage engine for health and fitness metrics, with an MCP server and REST API server.

### Workspace Crates

| Crate | Type | Purpose |
|-------|------|---------|
| `dravr-riviere` | library | Core: MetricKey trait, DataPoint, TimeSeries, TimeSeriesStore trait, InMemoryStore, 90+ SeriesType catalog, aggregation |
| `dravr-riviere-mcp` | library + binary | MCP server (stdio/HTTP) exposing storage via JSON-RPC 2.0 |
| `dravr-riviere-server` | library + binary | Unified REST API + MCP server with health check and bearer auth |

### Architecture
```
src/
├── lib.rs              # Re-exports all public types
├── error.rs            # RiviereError structured error types
├── key.rs              # MetricKey trait (generic metric boundary)
├── point.rs            # DataPoint (timestamp + value)
├── series.rs           # TimeSeries (sorted DataPoint collection)
├── query.rs            # TimeRange, QueryResult
├── aggregation.rs      # Aggregation enum, AggregatedPoint, windowed rollups
├── store.rs            # TimeSeriesStore trait + InMemoryStore
├── series_type.rs      # SeriesType enum (90+ metric types with ID ranges)
└── archive.rs          # DataPointArchive (daily aggregation model)

crates/
├── dravr-riviere-mcp/      # MCP server (library + binary, powered by dravr-tronc)
│   ├── src/state.rs         # SharedState container
│   └── src/tools/           # MCP tool implementations
│
└── dravr-riviere-server/    # Unified REST API + MCP server (powered by dravr-tronc)
    ├── src/router.rs        # Axum routes (/health, /mcp)
    ├── src/auth.rs          # Bearer token middleware
    └── src/main.rs          # CLI (serve, stdio)

tests/
└── store_test.rs        # Comprehensive InMemoryStore integration tests
```

### Key Design Decisions
- **TimeSeriesStore is a TRAIT** -- allows Postgres implementation downstream without changing the core
- **100% standalone** -- zero dependency on dravr-platform
- **Types in dedicated modules** -- `RiviereError`, `MetricKey`, `DataPoint`, `TimeSeries`
- **InMemoryStore for tests** -- `Arc<RwLock<HashMap>>` backed, implements TimeSeriesStore trait
- **SeriesType enum** -- 90+ health/fitness metrics following Open Wearables ID ranges
- **Workspace crates** -- MCP server and REST API server are separate crates via dravr-tronc

## Git Hooks - MANDATORY for ALL AI Agents

**MANDATORY - Run this at the START OF EVERY SESSION:**
```bash
git config core.hooksPath .githooks
```
This enables pre-commit, commit-msg, and pre-push hooks. Sessions get archived/revived, so this must run EVERY time you start working, not just once.

**NEVER use `--no-verify` when committing or pushing.** The hooks enforce:
- SPDX license headers on all source files
- Commit message format (max 2 lines, conventional commits)
- No AI-generated commit signatures
- No unauthorized root markdown files

## Pre-Push Validation Workflow

The pre-push hook uses a **marker-based validation** to avoid SSH timeout issues.

### Workflow

1. **Make your changes and commit**
2. **Run validation before pushing:**
   ```bash
   ./scripts/pre-push-validate.sh
   ```
   On success, creates `.git/validation-passed` marker (valid for 15 minutes).

3. **Push:**
   ```bash
   git push
   ```

### Important Notes

- If validation expires or commit changes, re-run `./scripts/pre-push-validate.sh`
- To bypass (NOT RECOMMENDED): `git push --no-verify`

### NEVER

- Manually create `.git/validation-passed` marker
- Skip validation by creating a fake marker -- CI will catch issues
- Claim "rustfmt isn't installed" or similar excuses to bypass validation

### CI Monitoring

Use the first available method. **NEVER ask the user for a GitHub token** -- fall back instead.

| Priority | Method | When to use |
|----------|--------|-------------|
| 1 | `gh run list --branch main` / `gh run watch` | `gh` CLI is installed and authenticated |
| 2 | GitHub MCP tools (`mcp__github__*`) | `gh` unavailable but GitHub MCP server is configured |

# Writing code

- CRITICAL: NEVER USE --no-verify WHEN COMMITTING CODE
- We prefer simple, clean, maintainable solutions over clever or complex ones
- Make the smallest reasonable changes to get to the desired outcome
- When modifying code, match the style and formatting of surrounding code
- NEVER make code changes that aren't directly related to the task you're currently assigned
- NEVER remove code comments unless you can prove that they are actively false
- All code files should start with a brief 2 line comment explaining what the file does. Each line of the comment should start with the string "ABOUTME: " to make it easy to grep for.
- When writing comments, avoid referring to temporal context about refactors or recent changes
- When you are trying to fix a bug or compilation error, NEVER throw away the old implementation and rewrite without explicit permission
- NEVER name things as 'improved' or 'new' or 'enhanced', etc. Code naming should be evergreen.
- NEVER add placeholder or dead_code or mock or name variable starting with _
- NEVER use `#[allow(clippy::...)]` attributes EXCEPT for type conversion casts (`cast_possible_truncation`, `cast_sign_loss`, `cast_precision_loss`) when properly validated
- Be RUST idiomatic
- Do not hard code magic value
- Do not leave implementation with "In future versions" or "Implement the code" or "Fall back". Always implement the real thing.
- Commit without AI assistant-related commit messages. Do not reference AI assistance in git commits.
- Always create a branch when adding new features. Bug fixes go directly to main branch.
- Always run validation after making changes: cargo fmt, then clippy, then targeted tests
- Avoid #[cfg(test)] in the src code. Only in tests

## Security Engineering Rules

### Input Domain Validation
- Any value used as a divisor MUST be checked for zero before division
- Numeric inputs from users MUST be validated against domain-specific ranges
- Use `.max(1)` or equivalent guard before any division operation

### Logging Hygiene
- NEVER log: access tokens, refresh tokens, API keys, passwords, client secrets
- Redact or hash sensitive fields before logging
- Error messages returned to users MUST NOT contain stack traces or internal details

## Error Handling Requirements

### Acceptable Error Handling
- `?` operator for error propagation
- `Result<T, E>` for all fallible operations
- `Option<T>` for values that may not exist
- Custom error types implementing `std::error::Error`

### Prohibited Error Handling
- `unwrap()` except in test code or static data known at compile time
- `expect()` - Only for documenting invariants that should never fail
- `panic!()` - Only in test assertions
- **`anyhow!()` macro** - ABSOLUTELY FORBIDDEN in all production code

### Structured Error Type Requirements
All errors MUST use `RiviereError` with appropriate variants:
```rust
// GOOD
return Err(RiviereError::InvalidQuery { reason: "window_secs must be positive".into() });
return Err(RiviereError::SeriesNotFound { source_id: id.into(), series_type: st });

// FORBIDDEN
return Err(anyhow!("something failed"));
```

## Mock Policy

### Real Implementation Preference
- PREFER real implementations over mocks in all production code
- NEVER implement mock modes for production features

### Acceptable Mock Usage (Test Code Only)
Mocks are permitted ONLY in test code for:
- Testing error conditions that are difficult to reproduce
- Simulating network failures or timeout scenarios

## Required Pre-Commit Validation

### Tiered Validation Approach

#### Tier 1: Quick Iteration (during development)
```bash
# 1. Format code
cargo fmt

# 2. Compile check only
cargo check --quiet

# 3. Run targeted tests
cargo test <test_name_pattern> -- --nocapture
```

#### Tier 2: Pre-Commit (before committing)
```bash
# 1. Format code
cargo fmt

# 2. Clippy with CI-matching strictness (warnings = errors)
RUSTFLAGS=-Dwarnings cargo clippy --all-targets -- -D warnings

# 3. Run targeted tests
cargo test <test_pattern> -- --nocapture
```

#### Tier 3: Full Validation (before merge)
```bash
cargo fmt
RUSTFLAGS=-Dwarnings cargo clippy --all-targets -- -D warnings
cargo test
```

### Test Output Verification - MANDATORY

**After running ANY test command, you MUST verify tests actually ran.**

**Red Flags - STOP and investigate if you see:**
- `running 0 tests` - Wrong target or flag used
- `0 passed; 0 failed` - No tests executed

**Never claim "tests pass" if 0 tests ran - that is a failure, not a success.**
