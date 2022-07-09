_default:
  @just --choose

dev:
  RUST_LOG=debug cargo run | bunyan

watch_dev:
  RUST_LOG=debug cargo watch --clear -x "run | bunyan"

checks:
  #!/usr/bin/env bash
  set -x
  cargo check
  cargo check --tests
  cargo clippy
  cargo fmt --all -- --check

tests:
  cargo test

tests_logs:
  TEST_LOG=enabled cargo test | bunyan

run:
  cargo run --release | bunyan
