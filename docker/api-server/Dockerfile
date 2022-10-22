FROM ghcr.io/rust-lang/rust:nightly-buster-slim as cargo-build

ARG RUSTFLAGS="--cfg tokio_unstable"

WORKDIR /toy

COPY pkg/ pkg/
COPY shared/ shared/
COPY apps/toyapi-d/ apps/toyapi-d/

WORKDIR /toy/apps/toyapi-d/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --path .

FROM gcr.io/distroless/cc

COPY --from=cargo-build /usr/local/cargo/bin/toyapi-d /usr/local/bin/toyapi-d

WORKDIR /toy

EXPOSE 3030
EXPOSE 9030

CMD ["/usr/local/bin/toyapi-d"]
