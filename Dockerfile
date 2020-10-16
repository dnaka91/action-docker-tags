FROM clux/muslrust:stable as builder

COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN cargo install --locked --path .

FROM scratch

COPY --from=builder /root/.cargo/bin/docker-tags /app/

ENTRYPOINT ["/app/docker-tags"]
