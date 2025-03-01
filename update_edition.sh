#!/bin/bash
# Script to update all crates to Rust 2024 edition

# List of all workspace members
MEMBERS=(
    "r3e"
    "r3e-api"
    "r3e-built-in-services"
    "r3e-config"
    "r3e-core"
    "r3e-deno"
    "r3e-endpoints"
    "r3e-event"
    "r3e-fhe"
    "r3e-neo-services"
    "r3e-oracle"
    "r3e-proc-macros"
    "r3e-runtime"
    "r3e-runlog"
    "r3e-scheduler"
    "r3e-secrets"
    "r3e-stock"
    "r3e-store"
    "r3e-tee"
    "r3e-worker"
    "r3e-zk"
)

for member in "${MEMBERS[@]}"; do
    if [ -f "$member/Cargo.toml" ]; then
        echo "Updating $member/Cargo.toml to edition 2024"
        
        # Replace the edition line with edition = "2024"
        sed -i '' 's/edition = "2021"/edition = "2024"/' "$member/Cargo.toml"
        
        # Add cargo-features at the top of the file
        # First, create a temporary file with the cargo-features line
        echo 'cargo-features = ["edition2024"]' > temp_cargo_header
        
        # Then prepend this to the existing Cargo.toml
        cat "$member/Cargo.toml" >> temp_cargo_header
        mv temp_cargo_header "$member/Cargo.toml"
    else
        echo "Warning: $member/Cargo.toml not found"
    fi
done

echo "All crates updated to Rust 2024 edition" 