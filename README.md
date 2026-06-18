# WaveSeed — wavespeed

WaveSeed (codename: wavespeed) is a prototype monorepo implementing a decentralized agricultural financing
protocol on Stellar + Soroban. It includes Soroban smart contract skeletons, a Next.js frontend scaffold, and a
NestJS backend scaffold with Prisma. This initial commit provides project structure, contract stubs, and developer
instructions for running on Stellar Testnet and iterating on the modules.

Modules included (scaffolded):
- `contracts/` — Soroban contracts: Farm Registry, Harvest Credit Tokenization, Lending Pool, Credit Scoring, Yield Oracle
- `frontend/` — Next.js + TypeScript + Tailwind scaffold (Stellar Wallet Kit + Freighter placeholders)
- `backend/` — NestJS + Prisma scaffold for metadata, KYC, analytics, and Horizon/Soroban indexing hooks
- `infra/` — Docker Compose, deployment scripts, and CI workflows

Next steps
- Review the Soroban contract stubs in `contracts/` and run `cargo test` (requires Soroban toolchain).
- Install frontend dependencies and run `pnpm install && pnpm dev` inside `frontend/`.
- Install backend and run `pnpm install && pnpm start:dev` inside `backend/`.

See the repository files for more details and development notes.
WaveSeed enables farmers to secure funding for planting seasons by tokenizing future harvests and connecting with global lenders on Stellar.
---

## Modules

| Module | Description |
|---|---|
| `contracts/farm-registry` | On-chain farmer identity and farm registration |
| `contracts/harvest-credit` | Tokenize future crop yields as credit assets |
| `contracts/lending-pool` | Escrow and liquidity management for lenders |
| `contracts/credit-scoring` | On-chain credit score based on repayment history |
| `contracts/yield-oracle` | Crop price feed for face value calculation |
| `frontend/` | Next.js + TypeScript + Tailwind UI with Freighter wallet |
| `backend/` | NestJS + Prisma for off-chain metadata, KYC, and indexing |
| `infra/` | Docker Compose, deployment scripts, CI workflows |

---

## Tech Stack

| Layer | Technology |
|---|---|
| Smart Contracts | Soroban (Rust), compiled to WASM |
| Frontend | Next.js 16, TypeScript, Tailwind CSS v4 |
| Wallet | [Freighter](https://freighter.app) + Stellar Wallet Kit |
| Backend | NestJS, Prisma ORM |
| Database | PostgreSQL |
| Network | Stellar Testnet / Mainnet |
| Infrastructure | Docker Compose, GitHub Actions CI |

---

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-stellar-cli)
- [Node.js 20+](https://nodejs.org/) + [pnpm](https://pnpm.io/)
- [Docker + Docker Compose](https://docs.docker.com/get-docker/)
- [Freighter Wallet](https://freighter.app) browser extension

### 1. Clone the repo

```bash
git clone https://github.com/fridaypetra55-afk/WaveSeed.git
cd WaveSeed
```

### 2. Smart Contracts

```bash
cd contracts
rustup target add wasm32-unknown-unknown
cargo test
cargo build --target wasm32-unknown-unknown --release
```

### 3. Frontend

```bash
cd frontend
pnpm install
cp .env.example .env.local   # fill in CONTRACT IDs and STELLAR_NETWORK
pnpm dev
# → http://localhost:3000
```

### 4. Backend

```bash
cd backend
pnpm install
cp .env.example .env         # fill in DATABASE_URL and Soroban config
pnpm start:dev
# → http://localhost:4000
```

### 5. Run with Docker

```bash
docker compose up --build
```

---

## Deploy Contracts

### Deploy to Testnet

```bash
./scripts/deploy.sh testnet <YOUR_SECRET_KEY>
```

Fund a testnet account via [Friendbot](https://friendbot.stellar.org) before deploying.

View deployed contracts on [Stellar Expert (Testnet)](https://stellar.expert/explorer/testnet).

---

## Environment Variables

Copy `.env.example` in both `frontend/` and `backend/` and fill in:

```env
# Stellar
STELLAR_NETWORK=testnet
HORIZON_URL=https://horizon-testnet.stellar.org
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org

# Contracts
NEXT_PUBLIC_FARM_REGISTRY_CONTRACT=C...
NEXT_PUBLIC_HARVEST_CREDIT_CONTRACT=C...
NEXT_PUBLIC_LENDING_POOL_CONTRACT=C...
NEXT_PUBLIC_YIELD_ORACLE_CONTRACT=C...

# Backend
DATABASE_URL=postgresql://user:password@localhost:5432/waveseed
```

---

## Project Structure
