FROM ghcr.io/rust-lang/rust:nightly-buster-slim as cargo-build

ARG RUSTFLAGS="--cfg tokio_unstable"

WORKDIR /toy

COPY pkg/ pkg/
COPY plugins/ plugins/
COPY shared/ shared/
COPY apps/actor/ apps/actor/
COPY docker/actor/ docker/actor/

WORKDIR /toy/apps/actor/actor-d

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --path .

FROM gcr.io/distroless/cc

COPY --from=cargo-build /usr/local/cargo/bin/actor-d /usr/local/bin/actor-d

WORKDIR /toy

EXPOSE 3031
EXPOSE 9031

CMD ["/usr/local/bin/actor-d", "subscribe"]
