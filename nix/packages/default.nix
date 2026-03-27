{ pkgs, ... }:
pkgs.callPackage (
  {
    lib,
    stdenv,
    trunk,
    binaryen,
    wasm-bindgen-cli,
    rustup,
  }:
  stdenv.mkDerivation {
    pname = "puntoboe";
    version = "0.1.0";
    src = lib.cleanSource ../..;
    buildInputs = [
      trunk
      binaryen
      wasm-bindgen-cli
      rustup
    ];
    buildPhase = ''
      rustup target add wasm32-unknown-unknown
      trunk build --release --dist $out
    '';
    installPhase = "true";
  }
) { }
