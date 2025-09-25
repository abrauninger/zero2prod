# Use the latest Rust stable release as base image
FROM rust:1.90.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]
