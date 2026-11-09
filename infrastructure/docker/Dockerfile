FROM rust:1.87-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system pixie \
    && useradd --system --gid pixie --no-create-home pixie \
    && mkdir -p /etc/pixie

COPY --from=builder /app/target/release/pixie /usr/local/bin/pixie
COPY web /usr/share/pixie/web
COPY infrastructure/config/config-pixie.yml /etc/pixie/config-pixie.yml

ENV PIXIE_CONFIG=/etc/pixie/config-pixie.yml
ENV PIXIE_WEB_ROOT=/usr/share/pixie/web

EXPOSE 8080

USER pixie

ENTRYPOINT ["/usr/local/bin/pixie"]
