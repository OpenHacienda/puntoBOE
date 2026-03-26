{ pkgs, ... }: {
  packages.default = pkgs.stdenv.mkDerivation {
    name = "modelo720-validator";
    src = ./.;
    buildInputs = with pkgs; [ trunk binaryen wasm-bindgen-cli rustup ];
    buildPhase = ''
      rustup target add wasm32-unknown-unknown
      trunk build --release --dist $out
    '';
    installPhase = "true";
  };
}
