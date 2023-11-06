#!/usr/bin/env bash
set -euxo pipefail

cargo test --all-features
cargo build --no-default-features
cargo build
cargo build --features futures
cargo build --features tokio
cargo build --all-features

RUSTDOCFLAGS="--cfg do_doc_cfg" cargo +nightly doc --all-features
lychee target/doc/byteorder2/index.html
