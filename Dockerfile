FROM rust:1.74-slim-bookworm as builder
LABEL authors="hazimani"

WORKDIR /opt/app

COPY Cargo.toml Cargo.lock /opt/app/
COPY src /opt/app/src

RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=target,target=/opt/prime-dx/build/target \
    cargo build --release

FROM debian:bookworm-slim as production

WORKDIR /opt/app

COPY --from=builder /opt/app/target/release/front-test /opt/app/front-test

EXPOSE 80

CMD ["./front-test", "--port", "80"]