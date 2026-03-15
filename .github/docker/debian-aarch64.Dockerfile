FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64

RUN rm -rf target/aarch64-unknown-linux-gnu && \
  apt-get update && \
  apt-get install -y gcc-aarch64-linux-gnu

RUN npm install -g corepack@latest && \
  corepack enable

RUN mkdir -p ~/.cargo && \
  touch ~/.cargo/config.toml

RUN echo "[target.aarch64-unknown-linux-gnu]" > ~/.cargo/config.toml && \
  echo "linker = \"aarch64-linux-gnu-gcc\"" >> ~/.cargo/config.toml