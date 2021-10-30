FROM ghcr.io/rust-lang/rust:nightly-buster-slim

WORKDIR /toy

COPY pkg/ pkg/
COPY shared/ shared/
COPY apps/console/console-backend/ apps/console/console-backend/

WORKDIR /toy/apps/console/console-backend/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/toy/apps/console/console-backend/target \
    cargo build --release

EXPOSE 3030

ENTRYPOINT ["/toy/apps/console/console-backend/target/release/console-backend"]
