{
  description = "A simple yet robust commandline random password generator.";
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, flake-compat }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
      rec {
        # `nix build`
        packages.genpass = naersk-lib.buildPackage {
          pname = "genpass";
          root = ./.;
        };
        defaultPackage = packages.genpass;

        # `nix run`
        apps.genpass = utils.lib.mkApp {
          drv = packages.genpass;
        };
        defaultApp = apps.genpass;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            cargo-edit
            cargo-outdated
            cargo-watch
            clippy
            git
            llvmPackages_13.llvm
            nixpkgs-fmt
            openssl
            openssh
            pkg-config
            rustc
            rustfmt
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      });
}
