FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64

RUN npm install -g corepack@latest && \
  corepack enable
