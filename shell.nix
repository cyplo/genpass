let
  pkgs = import <nixpkgs> {};
  unstableTarball = fetchTarball https://github.com/NixOS/nixpkgs-channels/archive/nixos-unstable.tar.gz;
  unstable = import unstableTarball {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      rustc rustfmt cargo cargo-watch unstable.cargo-release rustPackages.clippy
      git direnv
      zlib
    ];
  }
