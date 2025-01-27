FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:stable-slim

RUN addgroup --system producer
RUN adduser --system producer --ingroup producer
USER producer:producer

WORKDIR /app
COPY --from=builder /app/target/release/new-case ./producer
EXPOSE 3000
