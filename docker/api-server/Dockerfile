FROM ghcr.io/rust-lang/rust:nightly-buster-slim as cargo-build

WORKDIR /toy

COPY pkg/ pkg/
COPY shared/ shared/
COPY apps/console/console-backend/ apps/console/console-backend/

WORKDIR /toy/apps/console/console-backend/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --path .

FROM rust:slim-buster

COPY --from=cargo-build /usr/local/cargo/bin/console-backend /usr/local/bin/console-backend

WORKDIR /toy

EXPOSE 3030

CMD ["/usr/local/bin/console-backend"]
