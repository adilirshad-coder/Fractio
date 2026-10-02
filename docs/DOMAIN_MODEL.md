# Domain model

Core aggregate concepts: InvestorIdentity with multiple WalletAssociations; FounderEntity with FounderMemberships; Project with lifecycle and verification/listing workflow; Offering with configurable allocation; investment/position; escrow; SeedVesting; milestone/capital claim; graduation; indexed portfolio and balance projections. DTOs must not expose provider-specific or raw KYC data. Monetary values use integer minor units or integer token units.

Allocation draft default: 1,500 bps Seed, 4,500 curve, 1,500 capital formation, 2,500 open market. This is configurable and the source documents also state a conflicting 1,500 bps open-market value.

Verification has Application -> Documents -> FinalReview stages. KYC (`KycLevel`: None/Lite/Full) and KYB status remain separate domain concepts. The 0002 migration adds token supply/mint and editable JSON landing pages, raise target/deadline/asset, trades, FX rates, custodial ledger entries, withdrawals, payment method, and role assignments. PKR FX rates are provider data and never float-based money values. Project state transitions are mirrored in the on-chain and backend lifecycle helpers.

User roles are independent assignments, so one user may be both an investor and founder. Verification records use the `application`, `documents`, and `final_review` stages; founder KYB is represented separately from project review.
