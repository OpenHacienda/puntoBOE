{ pkgs, lib, ... }:
pkgs.stdenv.mkDerivation {
  name = "modelo720-validator";
  src = lib.cleanSource ../..;
  buildInputs = with pkgs; [ trunk binaryen wasm-bindgen-cli rustup ];
  buildPhase = ''
    rustup target add wasm32-unknown-unknown
    trunk build --release --dist $out
  '';
  installPhase = "true";
}
