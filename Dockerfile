FROM rust:1.80-bookworm as builder
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    binaryen \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install --locked cargo-leptos
WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo leptos build --release
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/your_app_name /app/
COPY --from=builder /app/target/site /app/site
EXPOSE 3000
ENV LEPTOS_SITE_ROOT=site
CMD ["/app/your_app_name"]
