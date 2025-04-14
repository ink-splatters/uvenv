{
  description = "Description for the project";

  inputs = {
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-analyzer-src.follows = "";
      };
    };
    crane.url = "github:ipetkov/crane";

    systems.url = "github:nix-systems/default";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  nixConfig = {
    extra-substituters = [
      # "https://aarch64-darwin.cachix.org"
      "https://nix-community.cachix.org"
      # "https://pre-commit-hooks.cachix.org"
    ];
    extra-trusted-public-keys = [
      # "aarch64-darwin.cachix.org-1:mEz8A1jcJveehs/ZbZUEjXZ65Aukk9bg2kmb0zL9XDA="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      # "pre-commit-hooks.cachix.org-1:Pkk3Panw5AW24TOv6kz3PvLhlH8puAsJTBbOPmBo7Rc="
    ];
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} (let
      systems = import inputs.systems;
    in {
      inherit systems;
      perSystem = {
        inputs',
        lib,
        pkgs,
        self',
        system,
        ...
      }: let
        inherit (inputs'.fenix.packages.minimal) toolchain;
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        uvenv = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
            doCheck = false;
          });
      in {
        checks = {
          inherit uvenv;

          cargo-fmt = craneLib.cargoFmt {
            inherit src;
          };
          taplo-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
          };

          cargo-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
              cargoNextestPartitionsExtraArgs = "--no-tests=pass";
            });
        };
        formatter = pkgs.alejandra;
        packages = {
          inherit uvenv;
          default = uvenv;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self'.checks;
          packages = [
            toolchain
            pkgs.maturin
            pkgs.uv
          ];
        };
      };
    });
}
