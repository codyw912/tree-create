<!-- dev-init:rust:start -->
## Rust Environment

- Use `just` recipes for routine project commands so humans and agents go through the same environment entry point.
- For ad hoc commands that still need the Rust dev shell, use `just dev <cmd...>`.
- If you need shell syntax like pipes, redirects, or `&&`, run it through `just dev-shell '<cmd>'`.
- Avoid running bare `cargo ...` unless you are already inside the project's devenv shell via `direnv`, `direnv exec`, or `nix develop`.

## Cross Building

- Keep cross toolchains out of the default shell unless this project needs them regularly.
- For Nix package builds, use `nix build .#packages.aarch64-linux.default` or `nix build .#packages.x86_64-linux.default` with local or remote builders configured.
- For Zig-based Rust builds, add `zig`, `cargo-zigbuild`, and the required Rust targets in `devenv.local.nix` or a project-specific cross profile.
- Do not add cross targets, Zig, or remote builder recipes to the default template unless every new Rust project should pay that cost on shell entry.
<!-- dev-init:rust:end -->
