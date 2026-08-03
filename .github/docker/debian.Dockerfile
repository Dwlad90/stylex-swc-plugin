FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian

RUN sed -i 's/node_20.x/node_24.x/' /etc/apt/sources.list.d/nodesource.list && \
  apt-get update && \
  apt-get install -y --no-install-recommends nodejs && \
  npm install -g corepack@latest && \
  corepack enable
