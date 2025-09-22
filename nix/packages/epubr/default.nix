# nix/packages/epubr/default.nix
{
  pkgs,
  inputs,
  system,
  ...
}: let
  naersk = inputs.naersk.lib.${system};
in
  naersk.buildPackage {
    # Let naersk read package.name + package.version from Cargo.toml
    src = ../../..; # repo root with Cargo.toml/Cargo.lock

    cargoBuildOptions = opts: opts ++ ["--locked"];

    nativeBuildInputs = with pkgs; [pkg-config];
    buildInputs = with pkgs; [openssl];

    # optional toggles you might want later:
    # doCheck = true;           # runs `cargo test` during build
    # CARGO_PROFILE = "release";
  }
