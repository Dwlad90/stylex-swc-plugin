FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64

RUN rm -rf target/aarch64-unknown-linux-gnu && \
  sed -i 's/node_20.x/node_24.x/' /etc/apt/sources.list.d/nodesource.list && \
  apt-get update && \
  apt-get install -y --no-install-recommends \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    nodejs

RUN npm install -g corepack@latest && \
  corepack enable

RUN mkdir -p ~/.cargo && \
  touch ~/.cargo/config.toml

RUN echo "[target.aarch64-unknown-linux-gnu]" > ~/.cargo/config.toml && \
  echo "linker = \"aarch64-linux-gnu-gcc\"" >> ~/.cargo/config.toml
