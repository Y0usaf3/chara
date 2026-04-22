FROM rust:1.92.0-trixie as builder

RUN apt-get update && apt-get install -y \
    curl unzip clang pkg-config libssl-dev build-essential \
    gcc-aarch64-linux-gnu g++-aarch64-linux-gnu \
    gcc-x86-64-linux-gnu g++-x86-64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y libssl-dev:arm64

ENV NVM_DIR /root/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.4/install.sh | bash \
    && . "$NVM_DIR/nvm.sh" \
    && nvm install 24 \
    && nvm use 24 \
    && nvm alias default 24

ENV PATH $NVM_DIR/versions/node/v24.14.1/bin:$PATH

RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:$PATH"

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup target add x86_64-unknown-linux-gnu

# Use build argument to specify target architecture
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Set environment variables based on target platform
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    echo "Building for ARM64"; \
    echo "CARGO_TARGET=aarch64-unknown-linux-gnu" >> /etc/environment; \
    echo "LINKER_CC=aarch64-linux-gnu-gcc" >> /etc/environment; \
    echo "LINKER_CXX=aarch64-linux-gnu-g++" >> /etc/environment; \
    else \
    echo "Building for AMD64"; \
    echo "CARGO_TARGET=x86_64-unknown-linux-gnu" >> /etc/environment; \
    echo "LINKER_CC=x86_64-linux-gnu-gcc" >> /etc/environment; \
    echo "LINKER_CXX=x86_64-linux-gnu-g++" >> /etc/environment; \
    fi

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
    CC_x86_64_unknown_linux_gnu=x86_64-linux-gnu-gcc \
    CXX_x86_64_unknown_linux_gnu=x86_64-linux-gnu-g++ \
    PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY package.json bun.lockb* ./
RUN bun install

COPY . .

# Build for both architectures (Leptos handles cross-compilation)
RUN cargo leptos build --release -v

# Runtime stage with dynamic platform selection
FROM debian:trixie-slim as runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl ca-certificates libgcc-s1 \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Copy binary - will be correct for the target platform
ARG TARGETPLATFORM
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    echo "Runtime: ARM64"; \
    else \
    echo "Runtime: AMD64"; \
    fi

COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/server /app/chara-arm64
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/server /app/chara-amd64
COPY --from=builder /app/target/site /app/site

# Use the appropriate binary based on platform
RUN if [ -f /app/chara-arm64 ]; then mv /app/chara-arm64 /app/chara; fi && \
    if [ -f /app/chara-amd64 ]; then mv /app/chara-amd64 /app/chara; fi && \
    rm -f /app/chara-arm64 /app/chara-amd64

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

CMD ["/app/chara"]
