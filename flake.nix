{
  description = "A simple yet robust commandline random password generator.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      inherit (pkgs) lib;

      craneLib = crane.lib.${system};
      src = craneLib.cleanCargoSource ./.;

      buildInputs =
        [
          # Add additional build inputs here
        ]
        ++ lib.optionals pkgs.stdenv.isDarwin [
          # Additional darwin specific inputs can be set here
          pkgs.libiconv
        ];

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src buildInputs;
      };

      # Build the actual crate itself, reusing the dependency
      # artifacts from above.
      my-crate = craneLib.buildPackage {
        inherit cargoArtifacts src buildInputs;
      };
    in {
      checks =
        {
          # Build the crate as part of `nix flake check` for convenience
          inherit my-crate;

          my-crate-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src buildInputs;
            cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
          };

          my-crate-doc = craneLib.cargoDoc {
            inherit cargoArtifacts src buildInputs;
          };

          my-crate-fmt = craneLib.cargoFmt {
            inherit src;
          };

          my-crate-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
          # the tests to run twice
          my-crate-nextest = craneLib.cargoNextest {
            inherit cargoArtifacts src buildInputs;
            cargoNextestExtraArgs = "--run-ignored all";
            partitions = 1;
            partitionType = "count";
          };
        }
        // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
        };

      packages.default = my-crate;

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        # Extra inputs can be added here
        nativeBuildInputs = with pkgs; [
          cacert
          cargo
          cargo-edit
          cargo-nextest
          cargo-outdated
          cargo-release
          cargo-watch
          clippy
          git
          llvmPackages_13.llvm
          nixpkgs-fmt
          openssh
          openssl
          pkg-config
          rustc
          rustfmt
        ];
      };
    });
}
