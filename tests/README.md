# RustGym Test Suite

This directory contains integration, end-to-end, and validation test scripts for the RustGym platform.

## Prerequisites

All test scripts require:

- `curl` and `jq` installed
- Bash 4.0+

Additional requirements vary by script (documented at the top of each file).

## Test Scripts

| Script                | Description                                                       | Requires Backend |   Requires Runner   |   Requires Docker   |
| --------------------- | ----------------------------------------------------------------- | :--------------: | :-----------------: | :-----------------: |
| `e2e_test.sh`         | Full end-to-end flow: register → login → browse → submit → verify |        ✓         | ✓ (for submissions) | ✓ (for submissions) |
| `correctness_test.sh` | Correctness properties: XP idempotency, level monotonicity, etc.  |        ✓         | ✓ (for submissions) | ✓ (for submissions) |
| `performance_test.sh` | Submission response time (< 15s)                                  |        ✓         |          ✓          |          ✓          |
| `validate_compose.sh` | Docker Compose YAML validation                                    |        ✗         |          ✗          |    ✓ (CLI only)     |
| `cli_test.sh`         | CLI tool build and commands                                       |        ✓         |          ✗          |          ✗          |

## Quick Start

### 1. Start infrastructure

```bash
# From the rustgym/ directory
docker compose up -d
```

### 2. Start the backend

```bash
cd backend
cargo run
```

### 3. Run tests

```bash
# Validate docker-compose (no running services needed)
bash tests/validate_compose.sh

# End-to-end tests
bash tests/e2e_test.sh

# Correctness properties
bash tests/correctness_test.sh

# Performance test (requires runner service)
bash tests/performance_test.sh

# CLI tool tests
bash tests/cli_test.sh
```

## Environment Variables

| Variable        | Default                 | Description                                     |
| --------------- | ----------------------- | ----------------------------------------------- |
| `BASE_URL`      | `http://localhost:3000` | Backend API base URL                            |
| `MAX_TIME_SECS` | `15`                    | Max submission response time (performance test) |
| `TEST_EMAIL`    | (auto-generated)        | Pre-existing test user email (CLI test)         |
| `TEST_PASSWORD` | (auto-generated)        | Pre-existing test user password (CLI test)      |

## Test Output

Each script outputs results in a consistent format:

- `✓ PASS` — test passed
- `✗ FAIL` — test failed (with detail)
- `⊘ SKIP` — test skipped (missing prerequisite)

Scripts exit with code 0 if all tests pass, 1 if any test fails.

## Correctness Properties Tested

1. **XP Idempotency** — Submitting the same task twice awards XP only once
2. **Level Monotonicity** — User level never decreases after XP is awarded
3. **Progress Consistency** — Completing a task updates task/level/quest progress
4. **Solution Code Never Exposed** — `GET /tasks/:slug` never returns `solution_code` or `test_code`
5. **Rate Limiting** — Returns 429 after exceeding rate limit (requires Redis)

## Notes

- Test scripts generate unique users per run (timestamped) to avoid conflicts
- Submission-based tests gracefully skip if the runner service is unavailable
- The `validate_compose.sh` script can run without any services running (only needs Docker CLI)
- The CLI test uses a temporary HOME directory to avoid polluting your real config
