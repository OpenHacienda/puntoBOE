{ pkgs, inputs, ... }:
(inputs.treefmt-nix.lib.evalModule pkgs {
  # anchor treefmt to the repo root
  projectRootFile = "flake.nix";

  # ── Nix pipeline (priority controls execution order within *.nix) ────────
  # 1. deadnix — remove dead bindings first so statix/nixfmt see clean code
  programs.deadnix.enable = true;
  programs.deadnix.no-lambda-pattern-names = true; # safe for callPackage style
  settings.formatter.deadnix.priority = 1;

  # 2. statix — fix anti-patterns before the final formatting pass
  programs.statix.enable = true;
  settings.formatter.statix.priority = 2;

  # 3. nixfmt — RFC-style canonical formatting as the last Nix pass
  programs.nixfmt.enable = true;
  settings.formatter.nixfmt.priority = 3;
  # ─────────────────────────────────────────────────────────────────────────

  # Rust — standard rustfmt
  programs.rustfmt.enable = true;

  # Leptos view! macros — must run after rustfmt
  programs.leptosfmt.enable = true;

  # TOML — Cargo.toml, flake-related manifests
  programs.taplo.enable = true;
}).config.build.wrapper
