{ pkgs ? import <nixpkgs> {} }:
with pkgs;

let
  myrustchannel = (rustChannelOf { date = "2018-07-24"; channel = "nightly"; });
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
    name = "screeps-rs-env";
    buildInputs = [
      git
      myrust
      pkgconfig
      openssl
      glib
      cairo
      pango
      gtk3
    ];
  }
