{ pkgs ? import <nixpkgs> {} }:
with pkgs;

let
  myrustchannel = (rustChannelOf { date = "2018-10-26"; channel = "nightly"; });
in
let
  myrust = (myrustchannel.rust.override {
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
    ];
  }
