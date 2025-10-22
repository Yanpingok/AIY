FROM rust:1.85-bullseye as build

ARG PROFILE=release
WORKDIR /work

RUN apt-get update && apt-get install -y cmake clang

COPY .git/ .git/
COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY aiy-execution aiy-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} --bin aiy-analytics-indexer

FROM debian:12-slim as deploy
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libjemalloc-dev
COPY --from=build --chmod=755 /work/target/release/aiy-analytics-indexer /opt/aiy/bin/aiy-analytics-indexer
