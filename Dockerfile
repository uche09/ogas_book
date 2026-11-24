FROM rust:1-slim-bookworm AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release -p ob-api


FROM debian:bookworm-slim AS runtime
LABEL org.opencontainers.image.source="https://github.com/uche09/ogas_book"
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home app
COPY --from=builder /app/target/release/ob-api /usr/local/bin/ob-api
USER app
EXPOSE 10000
CMD ["ob-api"]