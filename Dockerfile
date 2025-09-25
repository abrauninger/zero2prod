# Use the latest Rust stable release as base image
FROM rust:1.90.0 as builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.90.0 as runtime

WORKDIR /app
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime
COPY configuration.yaml configuration.yaml

ENTRYPOINT ["./target/release/zero2prod"]
