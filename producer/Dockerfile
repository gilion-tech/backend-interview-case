FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:stable-slim

RUN addgroup --system <group>
RUN adduser --system <user> --ingroup <group>
USER <user>:<group>

WORKDIR /app
COPY --from=builder /app/target/release/new-case ./producer
EXPOSE 3000
