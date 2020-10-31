let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  channel = (nixpkgs.rustChannelOf { rustToolchain = ./rust-toolchain; });
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "genpass_shell";
    buildInputs = [
      channel.rust
      cacert openssl openssh zlib
      llvm pkgconfig git
    ];
  }
