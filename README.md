<div align="center">

# Fractio

**The private micro-equity protocol. Invest in startups from $10, or raise from your own community.**

[fractio.shop](https://fractio.shop) · Built on Solana · Rust + Anchor · Next.js

![Status](https://img.shields.io/badge/status-demo%20platform-FFB800)
![Chain](https://img.shields.io/badge/chain-Solana-9945FF)
![Token](https://img.shields.io/badge/token-Token--2022-blue)

</div>

> ⚠️ **Fractio is currently a demonstration platform and is not licensed to offer securities.** Nothing in this repository is investment advice. Shares in early-stage companies are illiquid and high-risk. See [Compliance](#compliance--regulatory-status).

---

## What is Fractio?

Fractio is a two-sided marketplace that turns a founder's community (customers, users, supporters, even AI-driven "agentic" founders) into real, protocol-enforced equity stakeholders.

- **Investors** buy fractional equity in vetted early-stage startups from as little as **$10**, paying in PKR (Raast / ACH / e-wallets) or stablecoins (USDT / USDC).
- **Founders** raise capital from the people who already believe in them, instead of relying only on VC and accredited-angel channels.

Instead of trusting backend policy, Fractio hard-codes its trust guarantees into the transaction layer on Solana.

The brand mark is the **Golden Slice** (Mango Yellow `#FFB800`): one slice of a complete cap table, a reminder that a $10 stake is real ownership.

---

## Key Features

### For investors
- Registration with Lite KYC
- Custodial wallet (Privy) with total balance shown in **PKR** at the interbank FX rate
- Deposit via PKR (Raast / ACH / e-wallets) or on-chain USDT / USDC
- Explore and trade across project phases: **New**, **Bonding**, **Graduated**
- Withdraw to PKR (Raast / ACH / e-wallets), with optional on-chain withdrawal

### For founders and startups
- Registration as sole proprietor, partner, founder / co-founder, or agentic founder
- KYB + KYC
- Editable in-app startup landing page
- Listing submission (manual, white-glove at first; automated later)
- Creator fee dashboard: claimable fees per startup (PKR / USDT / USDC)
- Receive capital-formation slabs as valuation milestones are hit
- Publish updates (videos, articles) on the project page

### Protocol-enforced guardrails

| Guardrail | What it does |
| --- | --- |
| **3-stage verification** | Application → Documents → Final Review before a project can accept any funds |
| **Escrowed funds** | Investor capital is program-locked until targets are met and the founder's bank account is verified |
| **5% ownership cap** | Enforced via Token-2022 transfer hooks; the invest button blocks over-limit purchases |
| **All-or-nothing raises** | If the target is missed by the deadline, investors are refunded automatically |

---

## How a Launch Works

Every project follows the same standard supply distribution so valuations are comparable across the platform.

| Phase | Allocation | Mechanics |
| --- | --- | --- |
| **Seed** | 15% | Fixed-price sale. Non-transferable; vests linearly, released daily over 365 days (Streamflow). 80% of proceeds back the curve's real reserves. |
| **Bonding Curve** | 45% | Constant-product (K-constant, Uniswap-v2 style) curve with virtual reserves. Instantly tradable. Seed participants are barred from this phase. |
| **Capital Formation** | 15% | Single-sided LP positions released to founders when valuation milestones are reached (up to 50M PKR market cap). |
| **Open Market** | 25% | Activated on graduation. All participants can trade; vested seed tokens become freely tradable. |

**Graduation** is an atomic instruction. Once the curve allocation is sold, it migrates liquidity into a permanent AMM (Meteora DAMM v2 / Raydium / in-house pool), burns or locks LP tokens, and flips the open-market flag.

> Total token supply per project (100M vs 1B) is still to be finalized and applied uniformly.

---

## Architecture

```
┌──────────────┐     ┌─────────────────────┐     ┌──────────────────────┐
│   Frontend   │◄───►│  API / WS gateway   │◄───►│  Postgres + Redis    │
│  Next.js PWA │     │ (Node/Fastify or    │     │  (projects, KYC,     │
│  Privy wallet│     │  Rust/Axum)         │     │   hot state, cache)  │
└──────┬───────┘     └──────────▲──────────┘     └──────────────────────┘
       │                        │
       │                 ┌──────┴──────────┐
       │                 │ Indexer         │  Helius / Triton Yellowstone gRPC
       │                 │ (events → DB)   │
       │                 └──────▲──────────┘
       ▼                        │
┌──────────────────────────────┴──────────────────────────────────────┐
│                    Solana programs (Rust + Anchor)                  │
│  Launch/Factory · Seed · Bonding Curve · Capital Formation ·        │
│  Graduation / Open Market · Token-2022 transfer hooks               │
└──────────────────────────────────────────────────────────────────────┘
```

### Tech stack

| Layer | Tools |
| --- | --- |
| **Chain** | Solana (mainnet + devnet), Token-2022 |
| **Programs** | Rust, Anchor; Streamflow for vesting; Pyth / Switchboard for external prices |
| **Frontend** | Next.js 15 (App Router), React 19, TypeScript, Tailwind CSS, shadcn/ui, Privy, Lightweight Charts, Zustand / TanStack Query |
| **Backend** | Rust (Axum), REST/GraphQL + WebSockets, BullMQ jobs |
| **Data** | PostgreSQL, Redis, Cloudflare R2 / S3, IPFS (Pinata) |
| **Infra** | Helius / Triton RPC + gRPC streaming (Alchemy / QuickNode fallback), Jito bundles for critical txs |
| **Ops** | Squads multisig + timelocks, Sentry, Prometheus / Grafana |

### On-chain programs

| Program | Responsibility |
| --- | --- |
| **Launch / Factory** | Creates the mint, sets allocations, initializes PDAs for each phase |
| **Seed** | Fixed-price sale, escrow, vesting stream creation |
| **Bonding Curve** | Constant-product curve with virtual reserves; blocks seed tokens from trading |
| **Capital Formation** | Single-sided positions released on market-cap milestones; founder claims |
| **Graduation** | Atomic migration to the open-market AMM |

---

## Getting Started

> Replace the commands below with the ones that match your repo layout.

### Prerequisites

- Node.js 20+ and a package manager (`pnpm` / `npm`)
- Rust (stable) and the Solana CLI
- Anchor CLI
- PostgreSQL and Redis
- A Privy app ID and a Helius (or Triton) API key

### Installation

```bash
git clone https://github.com/adilirshad-coder/Fractio.git
cd Fractio
```

### Environment variables

Copy the example file and fill in your values:

```bash
cp .env.example .env
```

```env
# Solana
SOLANA_CLUSTER=devnet
RPC_URL=
HELIUS_API_KEY=

# Auth / wallets
NEXT_PUBLIC_PRIVY_APP_ID=
PRIVY_APP_SECRET=

# Data
DATABASE_URL=postgresql://user:pass@localhost:5432/fractio
REDIS_URL=redis://localhost:6379

# Storage
R2_BUCKET=
PINATA_JWT=

# Compliance
KYC_PROVIDER_KEY=
```

### Run locally

```bash
# 1. Build and test the on-chain programs
anchor build
anchor test

# 2. Start backend services
# (database migrations, API, indexer)

# 3. Start the frontend
# cd app && pnpm install && pnpm dev
```

---

## Project Structure

> Adjust to match the actual repo.

```
Fractio/
├── programs/            # Anchor programs (factory, seed, curve, capital formation, graduation)
├── tests/               # Anchor / LiteSVM integration tests
├── app/                 # Next.js frontend
├── backend/             # API, WebSocket gateway, jobs
├── indexer/             # Chain event ingestion
└── docs/                # Specs, executive summary, platform analysis
```

---

## Roadmap

- [ ] **Phase 1: MVP core.** Token factory, fixed-price seed, Streamflow vesting, basic bonding curve (seed tokens blocked)
- [ ] **Phase 2: Capital formation.** Single-sided LP logic and milestone-based founder claims
- [ ] **Phase 3: Graduation.** Atomic migration and open-market trading of vested seed tokens
- [ ] **Phase 4: Polish.** Real-time feeds, TradingView-style charts, discovery feed, anti-snipe measures, fee switches, analytics
- [ ] **Phase 5: Scale.** Dedicated RPC, advanced indexing, mobile (PWA → React Native / Expo), English + Urdu

---

## Security

- Mandatory audits (at least two reputable firms) for the launch, curve, capital-formation, and graduation programs
- Squads multisig and timelocks on all admin functions; no single key can rug
- Anchor unit and integration tests, LiteSVM, local validator, and fuzzing on curve math and vesting edge cases
- Real-time monitoring and rate limits on launches and large trades

Found a vulnerability? Please **do not open a public issue**. Email `security@fractio.shop` *(update with your real contact)*.

---

## Compliance & Regulatory Status

Fractio is a **demonstration platform** and is **not licensed to offer securities**. Public offerings of private-company equity are heavily regulated. Fractio is designed with Pakistan's **Virtual Assets Act 2026 (PVARA)** in mind, and with compatibility for regimes such as the UK Public Offer Platform in view. A licensing pathway (own license or licensed partner) and local legal counsel are required before any live public offering. KYC/AML hooks (e.g. Sumsub) are built in from the start.

---

## Contributing

1. Fork the repo and create a branch: `git checkout -b feature/your-feature`
2. Write tests for any program or curve-math changes
3. Open a pull request describing the change and its security implications

---

## License

*Add your license here (e.g. MIT, Apache-2.0, or proprietary).*

---

<div align="center">

**Fractio.** The high-performance bridge between community conviction and institutional-grade financial participation.

</div>