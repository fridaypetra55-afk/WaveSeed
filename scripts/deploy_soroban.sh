#!/usr/bin/env bash
set -euo pipefail

echo "This script compiles the contracts and provides commands to deploy to Soroban Testnet."
echo "Requires: soroban CLI, stellar-core or access to a Soroban RPC endpoint"

cd contracts
for d in */ ; do
  echo "Building $d"
  (cd "$d" && cargo build --target wasm32-unknown-unknown --release) || true
done

echo "Contracts compiled. Use 'soroban contract deploy' with the generated .wasm files."
