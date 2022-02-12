{
  description = "A simple yet robust commandline random password generator.";
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
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
            llvmPackages_13.llvm
            nixpkgs-fmt
            clippy
            rustc
            rustfmt
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      });
}
