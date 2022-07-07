_default:
  @just --choose

dev:
  RUST_LOG=debug cargo run | bunyan

checks:
  #!/usr/bin/env bash
  set -x
  cargo check
  cargo check --tests
  cargo clippy
  cargo fmt --all -- --check

tests:
  TEST_LOG=enabled cargo test | bunyan

run:
  cargo run --release | bunyan
