{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv/latest";
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
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "tree-create";
            version = "0.3.0";
            inherit src;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [ openssl ]
              ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [ libiconv zlib ];
          };
        }
      );

    };
}
