FROM --platform=$BUILDPLATFORM rust:1.92.0-trixie as builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Install base dependencies
RUN apt-get update && apt-get install -y \
    curl unzip clang pkg-config libssl-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install cross-compilation toolchain based on target
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    apt-get update && apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu && \
    dpkg --add-architecture arm64 && \
    apt-get update && apt-get install -y libssl-dev:arm64 && \
    rustup target add aarch64-unknown-linux-gnu && \
    rm -rf /var/lib/apt/lists/*; \
    else \
    rustup target add x86_64-unknown-linux-gnu && \
    rm -rf /var/lib/apt/lists/*; \
    fi

ENV NVM_DIR=/root/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.4/install.sh | bash \
    && . "$NVM_DIR/nvm.sh" \
    && nvm install 24 \
    && nvm use 24 \
    && nvm alias default 24

ENV PATH="$NVM_DIR/versions/node/v24.14.1/bin:$PATH"

RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:$PATH"

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown

# Set up cross-compilation for ARM64
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY package.json bun.lockb* ./
RUN bun install

COPY . .

# Build with cargo-leptos (will use native/target as appropriate)
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    LEPTOS_BIN_TARGET_TRIPLE=aarch64-unknown-linux-gnu cargo leptos build --release -vv; \
    else \
    cargo leptos build --release -vv; \
    fi

# Runtime stage
FROM debian:trixie-slim as runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl ca-certificates libgcc-s1 \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy artifacts based on target platform
ARG TARGETPLATFORM
COPY --from=builder /app/target/*/release/server /app/chara
COPY --from=builder /app/target/site /app/site

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

CMD ["/app/chara"]
