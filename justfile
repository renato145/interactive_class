_default:
  @just --choose

dev:
  RUST_LOG=debug cargo run | bunyan

watch_dev:
  RUST_LOG=debug cargo watch --clear -i "tests" -i "frontend" -x "run | bunyan"

refresh_bindings:
  rm frontend/bindings/*
  cargo test --lib
  cd frontend && pnpm run format-bindings

checks:
  #!/usr/bin/env bash
  set -x
  cargo check
  cargo check --tests
  cargo clippy --all-targets
  cargo fmt --all -- --check

tests:
  cargo test
  cd frontend && pnpm run format-bindings

tests_logs:
  TEST_LOG=enabled cargo test | bunyan
  cd frontend && pnpm run format-bindings

run:
  cargo run --release | bunyan
