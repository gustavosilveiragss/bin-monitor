FROM rust:latest as builder
RUN rustup default stable && rustup update
RUN rustup target add x86_64-unknown-linux-gnu
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-gnu --release

FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y libpq5
RUN rm -rf /var/lib/apt/lists/*

COPY target/x86_64-unknown-linux-gnu/release/bin-monitor-api /app/bin-monitor-api

CMD ["/app/bin-monitor-api", "--port", "8080"]