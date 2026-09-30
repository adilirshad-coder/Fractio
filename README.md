# Fractio backend and Solana protocol scaffold

Rust 2021, Axum, SQLx/PostgreSQL 16, Redis 7 and Anchor 0.31.1. The brief's detailed source documents are placeholders; product and legal decisions are tracked in `docs/ASSUMPTIONS.md`.

Copy `.env.example` to `.env`, start dependencies with `docker compose up postgres redis`, run `cargo run -p fractio-backend -- migrate`, then start with `cargo run -p fractio-backend`. API business routes require a configured HS256 bearer token and currently return `NOT_IMPLEMENTED`. Health routes are public.

This is architecture scaffolding, not a production financial protocol. No user funds should be accepted. Database records and Solana transactions cannot be committed atomically; production flows need outbox/idempotency, confirmation tracking and reconciliation. Never put raw KYC or private keys on-chain or in source control.
