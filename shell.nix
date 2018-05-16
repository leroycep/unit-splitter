{ pkgs ? import <nixpkgs> {} }:
with pkgs;

let
  myrustchannel = (rustChannelOf { date = "2018-04-19"; channel = "nightly"; });
in
let
  myrust = (myrustchannel.rust.override {
    extensions = [ "rust-std" ];
    targets = [
        "wasm32-unknown-unknown"
        "wasm32-unknown-emscripten"
        "asmjs-unknown-emscripten"
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
    ];
  }
