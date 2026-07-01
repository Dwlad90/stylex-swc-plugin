#!/usr/bin/env bash
# Fast local Playwright visual-snapshot loop for the StyleX SWC workspace.
#
# Strategy: keep a long-running Playwright container with the same image CI uses
# (mcr.microsoft.com/playwright:v<x.y.z>-noble). Cache node_modules, pnpm store,
# turbo cache, cargo target dir, and rust toolchain in named docker volumes so
# nothing has to be rebuilt between invocations. The Linux napi binary lives at
# crates/stylex-rs-compiler/dist/rs-compiler.linux-x64-gnu.node alongside the
# darwin one, so the host stays untouched. Auto-rebuild kicks in only when Rust
# source has changed since the last build.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

NODE_VERSION="${STYLEX_VISUAL_NODE_VERSION:-24.18.0}"
PNPM_VERSION="${STYLEX_VISUAL_PNPM_VERSION:-10.30.3}"
RUST_TOOLCHAIN="${STYLEX_VISUAL_RUST_TOOLCHAIN:-stable}"
REUSE_HOST_CACHES="${STYLEX_VISUAL_REUSE_HOST_CACHES:-true}"
# Default to linux/arm64 so local Apple Silicon Docker and the ARM64 GitHub
# visual jobs generate the same Linux snapshots without QEMU emulation.
PLATFORM="${STYLEX_VISUAL_PLATFORM:-linux/arm64}"
PLATFORM_SLUG="$(printf '%s' "${PLATFORM:-native}" | tr '/:' '--')"

if [ "$PLATFORM_SLUG" = "linux-arm64" ]; then
  DEFAULT_CONTAINER_NAME="stylex-visual"
else
  DEFAULT_CONTAINER_NAME="stylex-visual-$PLATFORM_SLUG"
fi
CONTAINER_NAME="${STYLEX_VISUAL_CONTAINER:-$DEFAULT_CONTAINER_NAME}"
VOLUME_PREFIX="${STYLEX_VISUAL_VOLUME_PREFIX:-$CONTAINER_NAME}"

VOL_NODE_MODULES="$VOLUME_PREFIX-node_modules"
VOL_PNPM_STORE="$VOLUME_PREFIX-pnpm-store"
VOL_TURBO="$VOLUME_PREFIX-turbo"
VOL_TARGET="$VOLUME_PREFIX-target"
VOL_CARGO_HOME="$VOLUME_PREFIX-cargo-home"
VOL_RUSTUP_HOME="$VOLUME_PREFIX-rustup-home"
VOL_APT="$VOLUME_PREFIX-apt-cache"

PNPM_STORE_MOUNT_PATH="/work/.pnpm-store"

LINUX_BINARY_DIR_REL="crates/stylex-rs-compiler/dist"

if [ -t 1 ]; then
  C_RESET=$'\033[0m'; C_DIM=$'\033[2m'; C_CYAN=$'\033[36m'
  C_YELLOW=$'\033[33m'; C_RED=$'\033[31m'; C_GREEN=$'\033[32m'
else
  C_RESET=; C_DIM=; C_CYAN=; C_YELLOW=; C_RED=; C_GREEN=
fi

log()  { printf '%s[snap]%s %s\n' "$C_CYAN"   "$C_RESET" "$*" >&2; }
ok()   { printf '%s[snap]%s %s\n' "$C_GREEN"  "$C_RESET" "$*" >&2; }
warn() { printf '%s[snap]%s %s\n' "$C_YELLOW" "$C_RESET" "$*" >&2; }
die()  { printf '%s[snap]%s %s\n' "$C_RED"    "$C_RESET" "$*" >&2; exit 1; }

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
}

require_cmd docker

read_playwright_version() {
  local lockfile="$ROOT/pnpm-lock.yaml"
  local version

  [ -f "$lockfile" ] || die "pnpm-lock.yaml not found at $lockfile"

  version=$(grep -m1 -oE "^[[:space:]]+playwright@[0-9]+\.[0-9]+\.[0-9]+:" "$lockfile" \
    | sed -E 's/^[[:space:]]+playwright@//; s/:$//' || true)
  printf '%s\n' "$version"
}

container_image() {
  local v
  v=$(read_playwright_version)
  [ -n "$v" ] || die "playwright not declared in pnpm-lock.yaml"
  echo "mcr.microsoft.com/playwright:v${v}-noble"
}

host_pnpm_store_path() {
  command -v pnpm >/dev/null 2>&1 || return 0
  pnpm store path 2>/dev/null
}

host_cargo_home() {
  if [ -n "${CARGO_HOME:-}" ] && [ -d "$CARGO_HOME" ]; then
    echo "$CARGO_HOME"
  elif [ -d "$HOME/.cargo" ]; then
    echo "$HOME/.cargo"
  fi
}

# Build the docker mount arg list. Bind-mounts host pnpm store + cargo
# registry/git when available (saves the "downloaded 1907" round-trip); falls
# back to named volumes otherwise. Platform-specific dirs (target, node_modules,
# turbo cache, rustup) always use named volumes.
build_mount_args() {
  MOUNT_ARGS=()
  MOUNT_ARGS+=( -v "$ROOT":/work )
  MOUNT_ARGS+=( -v "$VOL_NODE_MODULES":/work/node_modules )
  MOUNT_ARGS+=( -v "$VOL_TURBO":/work/.turbo )
  MOUNT_ARGS+=( -v "$VOL_TARGET":/work/target )
  MOUNT_ARGS+=( -v "$VOL_RUSTUP_HOME":/root/.rustup )
  MOUNT_ARGS+=( -v "$VOL_CARGO_HOME":/root/.cargo )
  MOUNT_ARGS+=( -v "$VOL_APT":/var/cache/apt )

  PNPM_STORE_MOUNT_PATH="/work/.pnpm-store"

  if [ "$REUSE_HOST_CACHES" = "true" ]; then
    local hps hch
    hps=$(host_pnpm_store_path)
    if [ -n "$hps" ] && [ -d "$hps" ]; then
      log "sharing host pnpm store: $hps"
      PNPM_STORE_MOUNT_PATH="/pnpm-store"
      MOUNT_ARGS+=( -v "$hps":"$PNPM_STORE_MOUNT_PATH" )
    else
      MOUNT_ARGS+=( -v "$VOL_PNPM_STORE":"$PNPM_STORE_MOUNT_PATH" )
    fi

    hch=$(host_cargo_home)
    if [ -n "$hch" ]; then
      [ -d "$hch/registry" ] && {
        log "sharing host cargo registry: $hch/registry"
        MOUNT_ARGS+=( -v "$hch/registry":/root/.cargo/registry )
      }
      [ -d "$hch/git" ] && {
        log "sharing host cargo git: $hch/git"
        MOUNT_ARGS+=( -v "$hch/git":/root/.cargo/git )
      }
    fi
  else
    MOUNT_ARGS+=( -v "$VOL_PNPM_STORE":"$PNPM_STORE_MOUNT_PATH" )
  fi
}

container_exists()  { docker container inspect "$CONTAINER_NAME" >/dev/null 2>&1; }
container_running() { [ "$(docker container inspect -f '{{.State.Running}}' "$CONTAINER_NAME" 2>/dev/null)" = "true" ]; }

expected_uname_for_platform() {
  case "$PLATFORM" in
    linux/amd64) echo "x86_64" ;;
    linux/arm64) echo "aarch64" ;;
    "")          echo "" ;;
    *)           echo "" ;;
  esac
}

container_uname() {
  docker exec "$CONTAINER_NAME" uname -m 2>/dev/null || true
}

validate_container_platform() {
  local expected actual
  expected=$(expected_uname_for_platform)
  [ -n "$expected" ] || return 0

  actual=$(container_uname)
  if [ -z "$actual" ]; then
    die "container '$CONTAINER_NAME' cannot execute for STYLEX_VISUAL_PLATFORM=$PLATFORM. \
Check that Docker can run this platform, then retry '$0 up'."
  fi

  if [ "$actual" != "$expected" ]; then
    die "container '$CONTAINER_NAME' is $actual, but STYLEX_VISUAL_PLATFORM=$PLATFORM expects $expected. \
Use the platform-scoped default container, or remove the stale one with: \
STYLEX_VISUAL_CONTAINER=$CONTAINER_NAME $0 nuke"
  fi
}

ensure_running() {
  if ! container_exists; then
    warn "container '$CONTAINER_NAME' not found — bootstrapping with: $0 up"
    cmd_up
    return
  fi
  if ! container_running; then
    log "starting '$CONTAINER_NAME'..."
    docker start "$CONTAINER_NAME" >/dev/null
  fi
  validate_container_platform
}

# Exec a bash command inside the container. Use a login shell so PATH picks up
# /root/.cargo/bin and the installed node. Host-side config is forwarded as
# SNAP_* env vars so the in-container script bodies stay single-quoted.
in_container() {
  docker exec -i \
    -e SNAP_NODE_VERSION="$NODE_VERSION" \
    -e SNAP_PNPM_VERSION="$PNPM_VERSION" \
    -e SNAP_RUST_TOOLCHAIN="$RUST_TOOLCHAIN" \
    "$@" "$CONTAINER_NAME" bash -lc "$EXEC_CMD"
}
ic() {
  EXEC_CMD="$1"
  shift
  in_container "$@"
}

# Map a uname -m value to the suffix napi-rs uses for the .node filename.
napi_suffix_for_arch() {
  case "$1" in
    x86_64)        echo "linux-x64-gnu" ;;
    aarch64|arm64) echo "linux-arm64-gnu" ;;
    *)             echo "" ;;
  esac
}

# Path to the .node binary the container will produce, based on its uname -m.
# Falls back to the x64 path if the container isn't running yet.
linux_binary_path() {
  local arch suffix
  if container_running; then
    arch=$(docker exec "$CONTAINER_NAME" uname -m 2>/dev/null || echo unknown)
  else
    case "$PLATFORM" in
      */arm64) arch=aarch64 ;;
      */amd64|"") arch=x86_64 ;;
      *) arch=x86_64 ;;
    esac
  fi
  suffix=$(napi_suffix_for_arch "$arch")
  [ -n "$suffix" ] || { echo ""; return; }
  echo "$ROOT/$LINUX_BINARY_DIR_REL/rs-compiler.${suffix}.node"
}

# Returns 0 if the Linux .node is missing OR older than any rust source.
linux_binary_stale() {
  local bin
  local newer_source
  bin=$(linux_binary_path)
  [ -n "$bin" ] || return 0
  [ -f "$bin" ] || return 0

  # Any *.rs or Cargo.* newer than the binary => stale.
  newer_source=$(find "$ROOT/crates" \
        \( -name target -o -name node_modules -o -name dist \) -prune -o \
        \( -name '*.rs' -o -name 'Cargo.toml' -o -name 'Cargo.lock' -o -name 'build.rs' \) \
        -newer "$bin" -print -quit 2>/dev/null)
  if [ -n "$newer_source" ]; then
    return 0
  fi
  if [ "$ROOT/Cargo.lock" -nt "$bin" ] 2>/dev/null; then
    return 0
  fi
  return 1
}

cmd_up() {
  local image
  image=$(container_image)

  if container_exists; then
    log "container '$CONTAINER_NAME' already exists; reusing"
    ensure_running
  else
    log "pulling $image (one-time, ~1.5GB)"
    docker pull "$image" >/dev/null

    build_mount_args
    log "creating container '$CONTAINER_NAME'"
    # Sleep infinity keeps it alive; the bind mount lets the container see live
    # source edits from the host.
    local platform_args=()
    [ -n "$PLATFORM" ] && platform_args=( --platform "$PLATFORM" )
    docker create \
      --name "$CONTAINER_NAME" \
      --label "stylex.visual.platform=${PLATFORM:-native}" \
      "${platform_args[@]}" \
      --ipc=host \
      --workdir /work \
      "${MOUNT_ARGS[@]}" \
      -e PLAYWRIGHT_BROWSERS_PATH=/ms-playwright \
      -e CI=true \
      -e TURBO_TELEMETRY_DISABLED=1 \
      -e DO_NOT_TRACK=1 \
      -e CARGO_HOME=/root/.cargo \
      -e RUSTUP_HOME=/root/.rustup \
      -e CARGO_INCREMENTAL=1 \
      -e RUSTUP_TOOLCHAIN="$RUST_TOOLCHAIN" \
      -e npm_config_store_dir="$PNPM_STORE_MOUNT_PATH" \
      -e PATH="/root/.cargo/bin:/work/node_modules/.bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" \
      "$image" \
      sleep infinity >/dev/null
    docker start "$CONTAINER_NAME" >/dev/null
  fi

  validate_container_platform

  log "ensuring system build tools (build-essential, pkg-config, libssl)"
  ic '
    set -e
    if ! dpkg -s build-essential >/dev/null 2>&1; then
      apt-get update -qq
      apt-get install -y --no-install-recommends \
        build-essential pkg-config libssl-dev ca-certificates curl git >/dev/null
    fi
  '

  log "ensuring rust toolchain ($RUST_TOOLCHAIN) via rustup"
  # shellcheck disable=SC2016 # $SNAP_* expand inside the container, not on the host
  ic '
    set -e
    if ! command -v rustup >/dev/null 2>&1; then
      curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain "$SNAP_RUST_TOOLCHAIN" --profile minimal
    fi
    rustup show active-toolchain >/dev/null
  '

  log "ensuring node $NODE_VERSION (direct install) + pnpm $PNPM_VERSION (corepack)"
  # shellcheck disable=SC2016 # $SNAP_* expand inside the container, not on the host
  ic '
    set -e
    if [ "$(node -v 2>/dev/null)" != "v${SNAP_NODE_VERSION}" ]; then
      case "$(uname -m)" in
        x86_64)         node_arch=x64 ;;
        aarch64|arm64)  node_arch=arm64 ;;
        *) echo "unsupported arch: $(uname -m)" >&2; exit 1 ;;
      esac
      tmp=$(mktemp -d)
      curl -fsSL "https://nodejs.org/dist/v${SNAP_NODE_VERSION}/node-v${SNAP_NODE_VERSION}-linux-${node_arch}.tar.xz" \
        -o "$tmp/node.tar.xz"
      tar -xJf "$tmp/node.tar.xz" -C /usr/local --strip-components=1
      rm -rf "$tmp"
    fi
    corepack enable
    corepack prepare "pnpm@${SNAP_PNPM_VERSION}" --activate
    node -v
    pnpm -v
  '

  log "installing workspace deps (--ignore-scripts; napi will be built separately)"
  ic '
    set -e
    pnpm install --frozen-lockfile --ignore-scripts
  '

  cmd_rebuild

  ok "ready. iterate with:  $0 run [<filter>]"
}

cmd_rebuild() {
  ensure_running
  log "building napi (incremental — first build is slow, later builds are cached)"
  ic '
    set -e
    pnpm --filter @stylexswc/rs-compiler run build
  '
  local bin
  bin=$(linux_binary_path)
  if [ -z "$bin" ] || [ ! -f "$bin" ]; then
    die "napi build finished but expected binary is missing: ${bin:-<unknown arch>}"
  fi
  ok "linux napi ready: ${bin#"$ROOT/"}"
}

# Returns docker exec flags for tty allocation, only when both stdin and stdout
# are real terminals (pnpm wraps stdio with pipes, so this is empty when invoked
# via `pnpm test:visual:update ...`).
tty_flags() {
  if [ -t 0 ] && [ -t 1 ]; then
    echo "-t"
  fi
}

# Kill leftover app dev servers from previous (often failed) runs that are still
# holding ports playwright's webServer wants to bind. Without this, the second
# run fails with "http://localhost:30xx is already used".
#
# Patterns are narrowed to entrypoints that actually bind ports — broad matches
# like "node /work/apps/" would also catch pnpm's own worker threads during an
# install. SIGTERM first so servers get to release sockets cleanly; SIGKILL as a
# fallback for anything still running after the grace window.
kill_leftover_servers() {
  docker exec "$CONTAINER_NAME" bash -c '
    set +e
    patterns=(
      "node .*/serve/build/main\\.js"
      "node .*/next/dist/bin/next"
      "node .*/\\.bin/(vite|rsbuild|rspack|farm)"
      "node .*/storybook/.*serve"
    )
    for p in "${patterns[@]}"; do
      pkill -TERM -f "$p" 2>/dev/null
    done
    sleep 1
    for p in "${patterns[@]}"; do
      pkill -KILL -f "$p" 2>/dev/null
    done
    # brief window for kernel to release TIME_WAIT sockets
    sleep 1
  ' >/dev/null 2>&1 || true
}

cmd_run() {
  ensure_running
  local filter="${1:-./apps/*}"

  if linux_binary_stale; then
    warn "rust source changed since last build — rebuilding napi"
    cmd_rebuild
  fi

  kill_leftover_servers
  log "updating snapshots ($filter)"
  # PLAYWRIGHT_UPDATE_SNAPSHOTS is declared in turbo.json's test:visual `env`
  # field, so setting it busts the task hash and replays through playwright
  # without needing --force. PLAYWRIGHT_BROWSERS_PATH is in passThroughEnv so
  # it reaches the task without affecting the hash.
  EXEC_CMD="pnpm test:visual \
    --filter='$filter' \
    --filter='!@stylexswc/rollup-large-example' \
    --continue"
  local tty
  tty=$(tty_flags)
  # shellcheck disable=SC2086 # word-split intentional: $tty is empty or "-t"
  in_container -e PLAYWRIGHT_UPDATE_SNAPSHOTS=true $tty
  ok "done. review changes with: git status -- '*/visual-tests/.playwright-snapshots/**'"
}

cmd_check() {
  ensure_running
  local filter="${1:-./apps/*}"

  if linux_binary_stale; then
    warn "rust source changed since last build — rebuilding napi"
    cmd_rebuild
  fi

  kill_leftover_servers
  log "running visual tests ($filter)"
  EXEC_CMD="pnpm test:visual \
    --filter='$filter' \
    --filter='!@stylexswc/rollup-large-example' \
    --continue"
  local tty
  tty=$(tty_flags)
  # shellcheck disable=SC2086 # word-split intentional: $tty is empty or "-t"
  in_container $tty
}

cmd_shell() {
  ensure_running
  log "opening shell in '$CONTAINER_NAME' (exit to leave)"
  docker exec -it "$CONTAINER_NAME" bash -l
}

cmd_down() {
  if container_running; then
    log "stopping '$CONTAINER_NAME' (volumes preserved)"
    docker stop "$CONTAINER_NAME" >/dev/null
    ok "stopped. resume with: $0 up"
  else
    log "container is already stopped"
  fi
}

cmd_nuke() {
  if container_exists; then
    log "removing container '$CONTAINER_NAME'"
    docker rm -f "$CONTAINER_NAME" >/dev/null
  fi
  log "removing named volumes"
  for v in "$VOL_NODE_MODULES" "$VOL_PNPM_STORE" "$VOL_TURBO" "$VOL_TARGET" \
           "$VOL_CARGO_HOME" "$VOL_RUSTUP_HOME" "$VOL_APT"; do
    docker volume rm "$v" >/dev/null 2>&1 || true
  done
  local bin
  bin=$(linux_binary_path)
  if [ -n "$bin" ] && [ -f "$bin" ]; then
    log "removing ${bin#"$ROOT/"}"
    rm -f "$bin"
  fi
  ok "clean. next $0 up starts from zero"
}

cmd_status() {
  printf '%scontainer%s   %s\n' "$C_DIM" "$C_RESET" "$CONTAINER_NAME"
  printf '%simage%s       %s\n' "$C_DIM" "$C_RESET" "$(container_image)"
  printf '%splatform%s    %s\n' "$C_DIM" "$C_RESET" "${PLATFORM:-native}"
  if container_exists; then
    if container_running; then
      printf '%sstate%s       %srunning%s\n' "$C_DIM" "$C_RESET" "$C_GREEN" "$C_RESET"
      local expected actual
      expected=$(expected_uname_for_platform)
      actual=$(container_uname)
      if [ -n "$actual" ]; then
        if [ -n "$expected" ] && [ "$actual" != "$expected" ]; then
          printf '%sarch%s        %s %s(expected %s)%s\n' \
            "$C_DIM" "$C_RESET" "$actual" "$C_RED" "$expected" "$C_RESET"
        else
          printf '%sarch%s        %s\n' "$C_DIM" "$C_RESET" "$actual"
        fi
      fi
    else
      printf '%sstate%s       %sstopped%s\n' "$C_DIM" "$C_RESET" "$C_YELLOW" "$C_RESET"
    fi
  else
    printf '%sstate%s       %smissing%s (run: %s up)\n' \
      "$C_DIM" "$C_RESET" "$C_RED" "$C_RESET" "$0"
  fi
  local bin
  bin=$(linux_binary_path)
  if [ -n "$bin" ] && [ -f "$bin" ]; then
    local size mtime
    size=$(du -h "$bin" | cut -f1)
    mtime=$(date -r "$bin" '+%Y-%m-%d %H:%M:%S' 2>/dev/null || stat -f '%Sm' "$bin")
    if linux_binary_stale; then
      printf '%slinux .node%s %s (%s, built %s) %s(STALE vs current rust source)%s\n' \
        "$C_DIM" "$C_RESET" "${bin##*/}" "$size" "$mtime" "$C_YELLOW" "$C_RESET"
    else
      printf '%slinux .node%s %s (%s, built %s) %s(fresh)%s\n' \
        "$C_DIM" "$C_RESET" "${bin##*/}" "$size" "$mtime" "$C_GREEN" "$C_RESET"
    fi
  else
    printf '%slinux .node%s %smissing%s (will build on next run)\n' \
      "$C_DIM" "$C_RESET" "$C_YELLOW" "$C_RESET"
  fi
}

usage() {
  cat <<EOF
Usage: $0 <command> [args]

Spin up a single Playwright container, cache everything in named docker
volumes, and iterate on visual snapshots in seconds.

Commands:
  up                  Create + start container, install rust/node/pnpm/deps,
                      build the linux napi binary. Idempotent.
  run    [<filter>]   Update snapshots. Defaults to ./apps/*. Auto-rebuilds the
                      napi if rust source changed since last build.
  check  [<filter>]   Run visual tests without updating snapshots.
  rebuild             Force-rebuild the linux napi binary.
  shell               Open an interactive shell in the container.
  status              Show container + linux .node freshness.
  down                Stop the container. Volumes are preserved.
  nuke                Remove container, volumes, and the linux .node binary.

Examples:
  $0 up
  $0 run @stylexswc/next-example
  $0 check @stylexswc/example-storybook
  $0 down

Troubleshooting:
  If "napi build finished but expected binary is missing" appears after moving
  between worktrees or rebasing, the existing container may still mount an old
  repo path. Remove the stale container, then recreate it:
  $0 nuke
  $0 up

  Or use a different container name:
  STYLEX_VISUAL_CONTAINER=stylex-visual-<name> $0 up

Env overrides:
  STYLEX_VISUAL_CONTAINER          container name (default: stylex-visual for
                                   linux/arm64, otherwise
                                   stylex-visual-<platform>)
  STYLEX_VISUAL_VOLUME_PREFIX      docker volume prefix (default: container name)
  STYLEX_VISUAL_NODE_VERSION       node version (default: 24.18.0)
  STYLEX_VISUAL_PNPM_VERSION       pnpm version (default: 10.30.3)
  STYLEX_VISUAL_RUST_TOOLCHAIN     rust toolchain (default: stable)
  STYLEX_VISUAL_PLATFORM           docker platform (default: linux/arm64 to
                                   match visual CI and Apple Silicon Docker)
  STYLEX_VISUAL_REUSE_HOST_CACHES  bind-mount host pnpm store + cargo
                                   registry/git instead of empty volumes
                                   (default: true)
EOF
}

main() {
  local cmd="${1:-}"
  [ $# -gt 0 ] && shift || true
  case "$cmd" in
    up)
      cmd_up "$@"
      ;;
    run)
      cmd_run "$@"
      ;;
    check)
      cmd_check "$@"
      ;;
    rebuild)
      cmd_rebuild "$@"
      ;;
    shell)
      cmd_shell "$@"
      ;;
    status)
      cmd_status "$@"
      ;;
    down)
      cmd_down "$@"
      ;;
    nuke)
      cmd_nuke "$@"
      ;;
    ""|-h|--help|help)
      usage
      ;;
    *)
      usage
      exit 1
      ;;
  esac
}

main "$@"
