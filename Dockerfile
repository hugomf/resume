# 1. This tells docker to use the Rust official image
FROM rust:1.78-buster as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/rust-server .
CMD ["/usr/local/bin/rust-server"]