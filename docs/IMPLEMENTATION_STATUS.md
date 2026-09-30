# Implementation status

## Implemented

Workspace manifests, Axum HTTP shell, environment settings, JWT bearer extraction, typed API errors, foundational domain types, integration ports, initial relational migration, Anchor project/config accounts and initialization, Token-2022/transfer-hook boundaries, Docker Compose, CI configuration and architecture documents.

## Stubbed by design

Investment/settlement, ownership cap, curve quotes, refunds, capital formation, creator fees, vesting settlement, graduation, compliance decisions and transfer permission. Indexer worker, Redis-backed jobs, WebSocket gateway, OpenAPI, concrete repositories/adapters, payment/webhook endpoints, granular RBAC/rate limits, and separate factory/seed/curve/capital/graduation Anchor deployables are not implemented.

## External integrations requiring configuration

Solana RPC/indexing, Privy or auth issuer, KYC/KYB, payment/bank rails, custody/KMS, Streamflow, storage, oracle, Meteora/Raydium, Redis, telemetry.

## Unresolved decisions

See `ASSUMPTIONS.md`.

## Verification

Not run in this environment: cargo fmt, cargo check, cargo test, cargo clippy, anchor build/test. The Rust toolchain was unavailable when checked. No successful compilation is claimed.
