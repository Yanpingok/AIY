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

RUN cargo build --profile ${PROFILE} --bin stress

FROM rust:1.85-bullseye
ARG PROFILE=release
ARG GIT_REVISION
COPY --from=builder /aiy/target/${PROFILE}/stress /usr/local/bin
WORKDIR "$WORKDIR/aiy"

RUN apt-get update && apt-get -y --no-install-recommends install wget=1.21-1+deb11u1 \
        iputils-ping netcat procps bind9-host bind9-dnsutils curl iproute2 git ca-certificates awscli

RUN git clone https://github.com/MystenLabs/sui.git ; \
        cd aiy ; \
        git checkout $GIT_REVISION ; \
        cd .. ; \
        mv aiy/* .
        
RUN mkdir -p /docker/stress
RUN cp docker/stress/entrypoint.sh /docker/stress/entrypoint.sh
RUN chmod +x /docker/stress/entrypoint.sh
RUN echo ${GIT_REVISION:-$GIT_REVISION} > /var/run/aiy_commit

CMD ["/docker/stress/entrypoint.sh"]
