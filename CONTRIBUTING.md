# Contributing to WaveSeed (wavespeed)

Thanks for your interest in contributing! This document explains how to get started contributing code, documentation, tests, and other improvements.

Getting started
- Fork the repo and create a feature branch: `git checkout -b feat/my-change`.
- Keep changes small and focused; one logical change per pull request.

Development
- Contracts: build Soroban contracts from `contracts/` with `cargo build --target wasm32-unknown-unknown --release` and test with the Soroban test harness.
- Backend: see `backend/README.md` for running the NestJS server and Prisma migrations.
- Frontend: see `frontend/README.md` to run the Next.js application.

Commit messages
- Use concise commit messages and reference issues when applicable. Example: `feat: add farm registration event`.

Testing
- Add unit tests for contract logic where possible.
- Add integration tests for end-to-end flows (mint → deposit → loan → repay).

Pull requests
- Open a PR against `main` and describe the change, testing steps, and any backward-incompatible impacts.
- Include tests and documentation updates when applicable.

Code style
- Keep style consistent with existing files. Use `rustfmt` for Rust, `prettier`/`eslint` for TypeScript.

Security disclosures
- If you discover a security issue, please open a private issue or contact repository maintainers rather than posting public details.

Maintainers
- Repository maintainers will review PRs and manage merges. Please be patient; reviews may take time.

Thank you for contributing!
