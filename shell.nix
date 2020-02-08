let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      rustc rustfmt cargo cargo-watch cargo-release rustPackages.clippy
      git direnv
      zlib
    ];
  }
