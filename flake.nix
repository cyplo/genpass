{
  description = "A simple yet robust commandline random password generator.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, crate2nix, ... }:
    let
      name = "genpass";
      rustChannel = "stable";
    in
    utils.lib.eachDefaultSystem
      (system:
        let
          # Imports
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlay
              (self: super: {
                # Because rust-overlay bundles multiple rust packages into one
                # derivation, specify that mega-bundle here, so that crate2nix
                # will use them automatically.
                rustc = self.rust-bin.${rustChannel}.latest.default;
                cargo = self.rust-bin.${rustChannel}.latest.default;
              })
            ];
          };
          inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
            generatedCargoNix;

          # Create the cargo2nix project
          project = pkgs.callPackage
            (generatedCargoNix {
              inherit name;
              src = ./.;
            })
            {
              # Individual crate overrides go here
              # Example: https://github.com/balsoft/simple-osd-daemons/blob/6f85144934c0c1382c7a4d3a2bbb80106776e270/flake.nix#L28-L50
              defaultCrateOverrides = pkgs.defaultCrateOverrides // {
                # The himalaya crate itself is overriden here. Typically we
                # configure non-Rust dependencies (see below) here.
                ${name} = oldAttrs: {
                  inherit buildInputs nativeBuildInputs;
                };
              };
            };

          buildInputs = with pkgs; [ openssl.dev cacert openssh zlib ];
          nativeBuildInputs = with pkgs; [ rustc cargo pkgconfig git ];
        in
        rec {
          packages.${name} = project.rootCrate.build;

          # `nix build`
          defaultPackage = packages.${name};

          # `nix run`
          apps.${name} = utils.lib.mkApp {
            inherit name;
            drv = packages.${name};
          };
          defaultApp = apps.${name};

          # `nix develop`
          devShell = pkgs.mkShell
            {
              inputsFrom = builtins.attrValues self.packages.${system};
              buildInputs = buildInputs ++ (with pkgs;
                [
                  nixpkgs-fmt
                  cargo-watch
                  cargo-edit
                  cargo-outdated
                  pkgs.rust-bin.${rustChannel}.latest.rust-analysis
                  pkgs.rust-bin.${rustChannel}.latest.rls
                  llvm
                ]);
              RUST_SRC_PATH = "${pkgs.rust-bin.${rustChannel}.latest.rust-src}/lib/rustlib/src/rust/library";
            };
        }
      );
}

