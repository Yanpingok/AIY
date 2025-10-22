# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.85-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/aiy"
RUN apt-get update && apt-get install -y cmake clang libpq5 libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY aiy-execution aiy-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin aiy-faucet \
    --bin aiy-bridge \
    --bin bridge-indexer \
    --bin deepbook-indexer \
    --bin aiy-bridge-cli \
    --bin aiy \
    --bin aiy-cluster-test \
    --bin aiy-tool

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/aiy"

# aiy-tool needs libpq at runtime and uses git when fetching move dependencies during build
RUN apt-get update && apt-get install -y libpq5 libpq-dev ca-certificates git

COPY --from=builder /aiy/target/release/aiy-faucet /usr/local/bin
COPY --from=builder /aiy/target/release/aiy-bridge /usr/local/bin
COPY --from=builder /aiy/target/release/bridge-indexer /usr/local/bin
COPY --from=builder /aiy/target/release/deepbook-indexer /usr/local/bin
COPY --from=builder /aiy/target/release/aiy-bridge-cli /usr/local/bin
COPY --from=builder /aiy/target/release/aiy /usr/local/bin
COPY --from=builder /aiy/target/release/aiy-cluster-test /usr/local/bin
COPY --from=builder /aiy/target/release/aiy-tool /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
