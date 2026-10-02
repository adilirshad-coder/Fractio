# Assumptions and unresolved decisions

The pasted requirements contain placeholders for source documents and explicit contradictions. Current configurable draft allocation follows Standard Launch Supply Distribution: 15% Seed, 45% Bonding Curve, 15% Capital Formation, 25% Open Market. Another passage gives 15% Open Market; do not treat either as finalized.

Unresolved decisions (defaults are configurable, not product commitments):

- A user may hold both investor and founder roles. Registration adds the requested role without removing existing roles.
- TODO(decision): Project list filters map `Explore` to all public statuses, `New` to `seed_live`, `Bonding` to `bonding_live`, and `Graduated` to `graduated`; product may revise these labels.
- Postgres lifecycle values are provisional until the chain submitter and indexer are operational; finalized chain state is authoritative.
- Development KYC uses a mock auto-approval provider only. Production must use a real provider or disable KYC routes.

- Select 100M or 1B total supply and the token decimals/base-unit interpretation.
- Resolve 15% versus 25% open-market allocation; current configurable default is 25% (15/45/15/25 bps).
- Decide whether raises are continuous bonding-curve sales or all-or-nothing escrow raises. The executive summary promises all-or-nothing raises with refunds and escrow, while the functional stack describes a Pump.fun-style curve.
- Define how the 5% beneficial-owner cap treats curve/pool/PDA-held tokens, transfer-hook exemptions, and multiple wallets controlled by one beneficial owner.
- Confirm the PVARA licensing and regulatory pathway before accepting funds or representing tokens as equity.
- Define tokenized equity legal meaning/authorization, graduation trigger, curve function/parameters, creator fee, capital-formation extraction formula, seed restrictions and vesting, Token-2022 extension/AMM compatibility, and 80% Seed proceeds backing mechanics.
- Select PKR rails, stablecoin representation, KYC/KYB provider and review policy, custodial wallet/signing provider, oracle sources, and admin/multisig thresholds.
