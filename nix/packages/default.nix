{ pkgs, inputs, ... }:
let
  # Stable Rust toolchain with the wasm32 target via fenix.
  # rustup cannot download toolchains inside the Nix sandbox.
  toolchain = inputs.fenix.packages.${pkgs.system}.combine [
    inputs.fenix.packages.${pkgs.system}.stable.cargo
    inputs.fenix.packages.${pkgs.system}.stable.rustc
    inputs.fenix.packages.${pkgs.system}.targets.wasm32-unknown-unknown.stable.rust-std
  ];
in
pkgs.stdenv.mkDerivation {
  pname = "puntoboe";
  version = "0.1.0";

  src = pkgs.lib.cleanSource ../..;

  nativeBuildInputs = [
    toolchain
    pkgs.rustPlatform.cargoSetupHook # sets up vendored Cargo deps in $CARGO_HOME
    pkgs.trunk
    pkgs.nodePackages.tailwindcss
    pkgs.wasm-bindgen-cli
    pkgs.binaryen
    pkgs.git # trunk uses git for project-root detection
  ];

  # Vendor all crates.io dependencies for an offline Cargo build.
  cargoVendorDir = pkgs.rustPlatform.importCargoLock {
    lockFile = ../../Cargo.lock;
  };

  buildPhase = ''
    runHook preBuild

    # Point trunk at the Nix-provided binaries so it never tries to download.
    # wasm-bindgen version (0.2.114) matches the crate in Cargo.lock exactly.
    cat > crates/app/Trunk.toml <<EOF
    [tools.wasm-bindgen]
    path = "$(which wasm-bindgen)"

    [tools.wasm-opt]
    path = "$(which wasm-opt)"
    EOF

    export TRUNK_CACHE_DIR=$(mktemp -d)

    cd crates/app
    trunk build --release --dist "$out"

    runHook postBuild
  '';

  installPhase = "true";
}
