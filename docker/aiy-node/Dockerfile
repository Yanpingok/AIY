# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.85-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/aiy"
RUN apt-get update && apt-get install -y cmake clang

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY aiy-execution aiy-execution
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} --bin aiy-node

# Production Image
FROM debian:bullseye-slim AS runtime
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y ca-certificates curl
ARG PROFILE=release
WORKDIR "$WORKDIR/aiy"
# Both bench and release profiles copy from release dir
COPY --from=builder /aiy/target/release/aiy-node /opt/aiy/bin/aiy-node
# Support legacy usages of /usr/local/bin/aiy-node
COPY --from=builder /aiy/target/release/aiy-node /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
