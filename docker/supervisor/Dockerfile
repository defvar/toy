FROM ghcr.io/rust-lang/rust:nightly-buster-slim as cargo-build

WORKDIR /toy

COPY pkg/ pkg/
COPY plugins/ plugins/
COPY shared/ shared/
COPY apps/supervisor/ apps/supervisor/
COPY docker/supervisor/ docker/supervisor/

WORKDIR /toy/apps/supervisor/supervisor-d

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install --path .

FROM gcr.io/distroless/cc

COPY --from=cargo-build /usr/local/cargo/bin/supervisor-d /usr/local/bin/supervisor-d

WORKDIR /toy

EXPOSE 3031

CMD ["/usr/local/bin/supervisor-d", "subscribe"]
#ENTRYPOINT ["./entrypoint.sh"]
