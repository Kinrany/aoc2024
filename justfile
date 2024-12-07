# The default command executed when running `just`
_default:
    @just --list --unsorted

# Build and test all solutions
all: rust-all

# Build and test all Rust solutions
rust-all:
    #!/usr/bin/env bash
    set -euxo pipefail

    cargo fmt --check --all;
    cargo build --workspace;
    cargo test --workspace;
    cargo clippy --workspace;

# Execute a Rust solution
rust-run dirpath:
    #!/usr/bin/env bash
    set -euxo pipefail

    cargo run --manifest-path {{dirpath}}/Cargo.toml < '{{dirpath}}/../input.txt'

# Create `new->old` symlinks for example and input files
ln new old:
    #!/usr/bin/env bash
    set -euxo pipefail

    ln -s ../{{old}}/input.txt ./{{new}}/input.txt;
    ln -s ../{{old}}/example.txt ./{{new}}/example.txt;
