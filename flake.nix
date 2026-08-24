{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv/latest";
    nix-config = {
      url = "git+ssh://git@github.com/codyw912/nix-config.git";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, rust-overlay, ... } @ inputs:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      forEachSystem = f: builtins.listToAttrs (map (name: { inherit name; value = f name; }) systems);
      mkPkgs = system: import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      cargoProject = builtins.pathExists ./Cargo.toml;
    in
    {
      devShells = forEachSystem (system:
        let
          pkgs = mkPkgs system;
        in
        {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              ./devenv.nix
            ];
          };
        }
      );

      packages = forEachSystem (system:
        let
          pkgs = mkPkgs system;
          src = pkgs.lib.cleanSourceWith {
            src = self;
            filter = path: type:
              let
                base = builtins.baseNameOf path;
              in
                !(pkgs.lib.elem base [
                  ".git"
                  ".direnv"
                  ".devenv"
                  "target"
                  "result"
                  "result-x86_64-linux-orb"
                  ".devcontainer.json"
                  ".pre-commit-config.yaml"
                ] || pkgs.lib.hasPrefix "result-" base);
          };
        in
        if cargoProject then
          {
            default = pkgs.rustPlatform.buildRustPackage {
              pname = "app";
              version = "0.1.0";
              inherit src;
              cargoLock.lockFile = ./Cargo.lock;
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [ openssl ]
                ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [ libiconv zlib ];
            };
          }
        else
          { }
      );

      # Template definition for nix flake init
      templates.default = {
        path = ./.;
        description = "Rust development environment with devenv";
        welcomeText = ''
          🦀 Rust development environment initialized!

          Files created:
          - devenv.nix: Development environment configuration
          - .cargo/config.toml: Cargo linker configuration for macOS
          - AGENTS.md: Agent command guidance for the project
          - flake.nix: Nix flake configuration
          - justfile: Common local Rust commands

          Next steps:
          1. Run 'direnv allow' to activate the environment
          2. For manual shell entry, use 'nix develop --no-pure-eval'
          3. Keep 'flake.nix' and 'flake.lock' tracked in git so direnv/nix-direnv can evaluate the shell
          4. Run 'just' to see common local recipes
          5. Use 'just dev <cmd...>' for ad hoc commands that need the project dev shell
          6. Run 'cargo init' to initialize a new Rust project (if needed)
        '';
      };
    };
}
