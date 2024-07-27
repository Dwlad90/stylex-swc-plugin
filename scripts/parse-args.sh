#!/usr/bin/env sh

PARAMS=""

build_rust=false
build_ts=false

while [ "$#" -ne 0 ]; do
  # shellcheck disable=2034
  case "$1" in
  -rust | --rust)
    build_rust=true
    shift
    ;;
  -ts | --ts | -typescript | --typescript)
    build_ts=true
    shift
    ;;
  # unsupported flags
  --* | -*=)
    echo "Error: Unsupported flag $1" >&2
    exit 1
    ;;
  # preserve positional arguments
  *)
    PARAMS="$PARAMS $1"
    shift
    ;;
  esac
done

# set positional arguments in their proper place
eval set -- "$PARAMS"
