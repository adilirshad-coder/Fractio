# Architecture

Rust 2021 workspace: Axum backend, SQLx/PostgreSQL persistence, Anchor program, REST boundary and replaceable ports for chain submission, signers, payments, compliance, vesting, curve engines and open markets. Dependency direction is API → application/domain ports ← adapters. The currently implemented backend is a foundation; adapter implementations, job workers, indexer and websocket gateway remain follow-on work.

Protocol lifecycle vocabulary: Draft → VerificationPending → Verified → SeedLive → SeedClosed → BondingLive → RaiseClosed → Graduating → Graduated, with Failed/Refundable/Cancelled paths. On-chain implemented operations are config and project initialization only; state-changing economic instructions return typed errors.

Database and Solana are not atomic together. Persist intent/idempotency/outbox in a PostgreSQL transaction, submit separately, then confirm and reconcile by signature.
