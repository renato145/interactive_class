#################
# 1 - FRONT-END #
#################
FROM node:16 as frontend
WORKDIR /app
COPY frontend frontend
WORKDIR /app/frontend
RUN npm install -g pnpm
RUN pnpm install --frozen-lockfile
RUN pnpm run build

##################
# 2 - RUST BUILD #
##################
FROM lukemathwalker/cargo-chef:latest-rust-1.62 as chef
WORKDIR /app

# Compute lock-file
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build project dependencies only
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin interactive_class

#####################
# 3 - Runtime stage #
#####################
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/interactive_class interactive_class
COPY --from=frontend /app/frontend/dist /app/frontend/dist
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./interactive_class"]
