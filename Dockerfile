FROM clux/muslrust:stable as builder

WORKDIR /volume

COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN cargo install --locked --path . && \
    strip --strip-all /root/.cargo/bin/docker-tags

FROM scratch

LABEL org.opencontainers.image.source https://github.com/dnaka91/action-docker-tags

COPY --from=builder /root/.cargo/bin/docker-tags /app/

ENTRYPOINT ["/app/docker-tags"]
