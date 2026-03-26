{ pkgs, ... }: {
  devShell = pkgs.mkShell {
    packages = with pkgs; [
      rustup
      trunk
      binaryen
      wasm-bindgen-cli
    ];

    shellHook = ''
      rustup target add wasm32-unknown-unknown
      rustup component add rust-analyzer
    '';
  };
}
