FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian

RUN npm install -g corepack@latest && \
  corepack enable
