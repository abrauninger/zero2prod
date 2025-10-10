# BACKEND

FROM lukemathwalker/cargo-chef:latest-rust-1.90.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y


FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as backend
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Up to this point, if our dependency tree stays the same, all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true

# Build the backend
RUN cargo build --release --bin zero2prod


# FRONTEND
FROM node:lts-alpine as frontend
WORKDIR /app
COPY frontend/package*.json ./
RUN npm install
COPY frontend .
RUN npm run build


# FINAL IMAGE
FROM debian:bookworm-slim as runtime
WORKDIR /app

# Install OpenSLL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend /app/target/release/zero2prod zero2prod
COPY --from=frontend /app/dist dist

COPY configuration configuration

ENV APP_ENVIRONMENT production
# TODO: Specify 'ENV RUST_BACKTRACE full'

ENTRYPOINT ["./zero2prod"]
