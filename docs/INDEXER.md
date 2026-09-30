# Indexer

Persist `(signature,event_index)` idempotently, checkpoint finalized slots, retain raw event payloads and derive portfolio/market projections. Handle replay, RPC gaps and fork rollback by finalized commitment and reconciliation. This repository currently has event/projection schema only; a worker and RPC adapter are not yet implemented.
