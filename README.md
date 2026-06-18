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
