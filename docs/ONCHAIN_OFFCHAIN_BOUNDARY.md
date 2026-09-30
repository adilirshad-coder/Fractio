# On-chain/off-chain boundary

Solana owns finalized protocol state, token settlement, reserves, and immutable events. Backend owns accounts, provider workflow references, documents/media, fiat/payment workflow, review/audit operations, search, notifications and projections. Never place raw identity, bank or KYC data on-chain. A database projection is never authoritative for token balances or transaction finality.
