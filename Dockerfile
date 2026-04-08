FROM rust:1.87-bookworm AS builder

WORKDIR /app

COPY pixie ./pixie

RUN cargo build --manifest-path pixie/Cargo.toml --release --locked

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system pixie \
    && useradd --system --gid pixie --no-create-home pixie

COPY --from=builder /app/pixie/target/release/pixie /usr/local/bin/pixie
COPY web /usr/share/pixie/web

ENV PIXIE_ADDR=0.0.0.0:8080
ENV PIXIE_THREADS=4
ENV PIXIE_WEB_ROOT=/usr/share/pixie/web

EXPOSE 8080

USER pixie

ENTRYPOINT ["/usr/local/bin/pixie"]
CMD ["serve"]
