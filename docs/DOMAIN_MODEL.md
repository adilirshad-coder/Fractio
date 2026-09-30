# Domain model

Core aggregate concepts: InvestorIdentity with multiple WalletAssociations; FounderEntity with FounderMemberships; Project with lifecycle and verification/listing workflow; Offering with configurable allocation; investment/position; escrow; SeedVesting; milestone/capital claim; graduation; indexed portfolio and balance projections. DTOs must not expose provider-specific or raw KYC data. Monetary values use integer minor units or integer token units.

Allocation draft default: 1,500 bps Seed, 4,500 curve, 1,500 capital formation, 2,500 open market. This is configurable and the source documents also state a conflicting 1,500 bps open-market value.
