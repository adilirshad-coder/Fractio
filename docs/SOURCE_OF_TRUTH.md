# Source of truth

| Data | Authority | Projection |
|---|---|---|
| Protocol lifecycle, mint and settled positions | Finalized Solana state | PostgreSQL indexer projection |
| Curve reserves / transaction finality | Solana RPC and program accounts | Cached API read model |
| Identity/KYC decision | Provider and backend review process | Provider reference/status |
| Project description/content | PostgreSQL/object storage | Search index/cache |
| PKR/fiat payment | Payment/bank provider | Reconciled payment records |
| Beneficial owner across multiple wallets | Unresolved identity policy | Mapping projection only |

Until the on-chain chain submitter and indexer exist, a project status recorded in Postgres is provisional. Finalized Solana state is authoritative for protocol lifecycle and settled positions.
