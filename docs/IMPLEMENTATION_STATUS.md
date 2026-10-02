# Implementation status

## Completed in this change

- Added the requested release profile settings: overflow checks, fat LTO, one codegen unit, and optimized non-incremental build overrides.
- Added a separate `anchor-build` CI job for pushes and pull requests, pinned to Solana CLI 2.1.21 and Anchor 0.31.1. It caches Cargo, AVM, and Solana files, checks the synchronized program ID, runs `cargo test -p fractio`, and runs `anchor build`. The program ID script compares source/configuration constants and is unaffected by Anchor generating a throwaway keypair in CI.
- Anchor lifecycle now requires explicit Draft -> VerificationPending submission before admin approval; supply presets are checked and pause toggling is idempotent.
- Backend and on-chain lifecycle tables have matching expected-pair tests.
- Program ID references match the public key derived from the existing local deployment keypair, and a CI check script has been added. `anchor keys sync` itself remains unrun because Anchor is unavailable here.
- Added migration 0003, database-backed role lookup, exact role matching, registration and `/v1/me`, grant-role/dev-token CLI commands, and authenticated public project listing with base64 keyset cursors.
- Added `KYC_PROVIDER` validation, a production guard against the mock provider, and a mock auto-approval adapter.
- Updated lifecycle, deployment, API, domain, security, assumptions, source-of-truth, and local setup documentation.

## Incomplete

`Cargo.lock` remains the existing 12-byte placeholder because `cargo generate-lockfile` could not run: Cargo is not installed or on PATH. The ignore rules do not contain a Cargo.lock rule. The conditional workspace compatibility check is inconclusive because neither Cargo nor Anchor/Solana is available, so no dependency pinning or workspace restructuring was attempted. Formatting, compilation, and test results remain unverified.

Backend KYC/KYB handlers, founder entity/project creation and editing, listing submission, ordered admin verification review, project detail visibility, DB integration tests, and a complete curl walkthrough remain unimplemented. The mock provider is not wired to a route yet; existing relevant handlers still return `NOT_IMPLEMENTED`. CI has a PostgreSQL service, but the ignored integration-test suite has not been added yet.

The canonical ID derived from the public half of `target/deploy/fractio-keypair.json` is `2ZwVTBRrzyTmzm5UJSoPjuvJYr1cr9qWkzWEqJq5WFBN`. All tracked references match. The keypair itself was not modified or printed.

## Verification

Commands were attempted on 2026-10-01 in the project root. Failures below are tool availability failures, not build/test failures.

| Command | Result |
|---|---|
| `cargo generate-lockfile` | NOT RUN: Cargo is not recognized; placeholder lockfile retained. |
| `cargo metadata --locked` | NOT RUN: Cargo is not recognized. |
| `cargo fmt --all -- --check` | NOT RUN: Cargo is not recognized. |
| `cargo check --workspace --all-targets` | NOT RUN: Cargo is not recognized; workspace conflict status is inconclusive. |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN: Cargo is not recognized. |
| `cargo test --workspace` | NOT RUN: Cargo is not recognized. |
| `cargo test -p fractio` | NOT RUN: Cargo is not recognized. |
| `cargo clippy -p fractio --all-targets -- -D warnings` | NOT RUN: Cargo is not recognized. |
| `cargo test -p fractio-backend -- --include-ignored` | NOT RUN: Cargo is not recognized. |
| `anchor build` | NOT RUN: Anchor is not recognized. |
| `cargo build-sbf --manifest-path programs/fractio/Cargo.toml` | NOT RUN: Cargo is not recognized (Anchor fallback). |
| `anchor keys list` / `anchor keys sync` | NOT RUN: Anchor is not recognized. |
| `bash scripts/check-program-id.sh` | NOT RUN: the available Windows bash launcher could not execute. |
| Static PowerShell comparison of program ID references | PASS: all tracked references, including both Anchor clusters, match the keypair public key. |
| `docker compose up -d postgres` | NOT RUN: Docker is not recognized. |
| `cargo run -p fractio-backend -- migrate` (twice) | NOT RUN: Cargo is not recognized; no database was started. |
| `docker compose config` | NOT RUN: Docker is not recognized. |
| `docker build .` | NOT RUN: Docker is not recognized. |

## Open decisions

- `programs/fractio/src/lib.rs`: curve and settlement behavior remain intentionally unimplemented.
- `programs/fractio/src/lib.rs`: milestone slabs may be claimable during bonding; current stub requires RaiseClosed.
- `docs/TOKEN_MODEL.md`: select token decimals and the mapping from whole-token supply to base units.
- `docs/ASSUMPTIONS.md`: multi-role identity, public-project filter labels, provisional Postgres status, mock KYC policy, and other product/legal/economic decisions listed there.
