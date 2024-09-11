{
  description = "A CLI to generate vim/nvim help doc from emmylua";

  nixConfig = {
    extra-substituters = "https://mrcjkb.cachix.org";
    extra-trusted-public-keys = "mrcjkb.cachix.org-1:KhpstvH5GfsuEFOSyGjSTjng8oDecEds7rbrI96tjA4=";
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    git-hooks = {
      # TODO: https://github.com/cachix/git-hooks.nix/pull/396
      # url = "github:cachix/git-hooks.nix";
      url = "github:mrcjkb/git-hooks.nix?ref=clippy";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    git-hooks,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      perSystem = {system, ...}: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            self.overlays.default
          ];
        };
        git-hooks-check = git-hooks.lib.${system}.run {
          src = self;
          hooks = {
            alejandra.enable = true;
            rustfmt.enable = true;
            clippy = {
              enable = true;
              settings = {
                denyWarnings = true;
                allFeatures = true;
              };
              extraPackages = with pkgs.vimcats; buildInputs ++ nativeBuildInputs;
            };
            cargo-check.enable = true;
          };
          settings = {
            rust.check.cargoDeps = pkgs.rustPlatform.importCargoLock {
              lockFile = ./Cargo.lock;
            };
          };
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "vimcats devShell";
          buildInputs =
            pkgs.vimcats.buildInputs
            ++ pkgs.vimcats.nativeBuildInputs
            ++ self.checks.${system}.git-hooks-check.enabledPackages
            ++ (with pkgs; [
              rust-analyzer
            ]);
          inherit (git-hooks-check) shellHook;
          doCheck = false;
        };

        packages = rec {
          default = vimcats;
          inherit (pkgs) vimcats;
        };

        checks = rec {
          default = git-hooks-check;
          inherit git-hooks-check;
        };
      };
      flake = {
        overlays.default = final: prev: {
          vimcats = final.rustPlatform.buildRustPackage {
            pname = "vimcats";

            src = self;

            version = ((final.lib.importTOML "${self}/Cargo.toml").package).version;

            cargoLock = {
              lockFile = ./Cargo.lock;
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
