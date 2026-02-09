#!/bin/bash

# 🏛️ Project Bazaar | Deployment Script
# Target: Stellar Futurenet

echo "--------------------------------------------------"
echo "🚀 Project Bazaar: Initiating Launch Sequence..."
echo "--------------------------------------------------"

# 1. Build the Contract (WASM)
echo "🏗️  Building Trust Logic Core..."
cargo build --target wasm32-unknown-unknown --release

# 2. Deploy to Futurenet
echo "📡 Uploading to Stellar Futurenet..."
# Note: Ensure you have an identity configured (e.g., 'alice') via `soroban config identity generate alice`
CONTRACT_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/project_bazaar.wasm \
    --source alice \
    --network futurenet)

echo "✅ Deployment Complete!"
echo "📝 Contract ID: $CONTRACT_ID"
echo "$CONTRACT_ID" > contract_id.txt