_default:
  @just --choose

dev:
  RUST_LOG=debug cargo run | bunyan

watch-dev:
  RUST_LOG=debug cargo watch --clear -i "tests" -i "frontend" -x "run | bunyan"

refresh-bindings:
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

tests-logs:
  TEST_LOG=enabled cargo test | bunyan
  cd frontend && pnpm run format-bindings

build: refresh-bindings
  cargo build --release
  cd frontend && pnpm run build

docker-build:
  docker build -t interactive-class .

run:
  cargo run --release | bunyan
