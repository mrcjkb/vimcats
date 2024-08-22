{
  description = "A CLI to generate vim/nvim help doc from emmylua";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem = {
        config,
        system,
        ...
      }: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            self.overlays.default
          ];
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "vimcats devShell";
          buildInputs = with pkgs;
          with pkgs.rustPlatform; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            alejandra
          ];
        };

        packages = rec {
          default = lemmy-help;
          inherit (pkgs) lemmy-help;
        };
      };
      flake = {
        overlays.default = final: prev: {
          cats-doc = final.rustPlatform.buildRustPackage {
            pname = "lemmy-help";

            src = self;

            version = ((final.lib.importTOML "${self}/Cargo.toml").package).version;
            cargoDeps = prev.rustPlatform.importCargoLock {
              lockFile = self + "/Cargo.lock";
            };

            buildFeatures = ["cli"];

            meta = with final.lib; {
              description = "CLI for generating vimdoc from LuaCATS annotations";
              license = with licenses; [mit];
              mainProgram = "vimcats";
            };
          };
        };
      };
    };
}
