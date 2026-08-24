set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
  just --list

@dev +args:
  if [[ -n "${DEVENV_PROFILE:-}" || -n "${IN_NIX_SHELL:-}" ]]; then \
    {{args}}; \
  elif command -v direnv >/dev/null 2>&1 && [[ -f .envrc ]]; then \
    direnv exec . {{args}}; \
  else \
    nix develop --no-pure-eval -c {{args}}; \
  fi

@dev-shell command:
  if [[ -n "${DEVENV_PROFILE:-}" || -n "${IN_NIX_SHELL:-}" ]]; then \
    bash -lc {{quote(command)}}; \
  elif command -v direnv >/dev/null 2>&1 && [[ -f .envrc ]]; then \
    direnv exec . bash -lc {{quote(command)}}; \
  else \
    nix develop --no-pure-eval -c bash -lc {{quote(command)}}; \
  fi

run *args:
  just --justfile "{{justfile()}}" dev cargo run -- {{args}}

build:
  just --justfile "{{justfile()}}" dev cargo build

check:
  just --justfile "{{justfile()}}" dev cargo check

test:
  just --justfile "{{justfile()}}" dev cargo test

fmt:
  just --justfile "{{justfile()}}" dev cargo fmt --all

lint:
  just --justfile "{{justfile()}}" dev cargo clippy --all-targets --all-features -- -D warnings
