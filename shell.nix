{ pkgs ? import <nixpkgs> {} }:
with pkgs;

let
  myrust = (rustChannels.stable.rust.override {
    extensions = [ "rust-std" ];
    targets = [
        "wasm32-unknown-unknown"
    ];
  });
in
  stdenv.mkDerivation {
    name = "unit-splitter-env";
    buildInputs = [
      git
      myrust
      pkgconfig
      openssl
      gcc
      libstdcxx5
    ];
  }
