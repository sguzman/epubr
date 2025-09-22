{ pkgs, perSystem, ... }:
# Use numtide/devshell's mkShell
perSystem.devshell.mkShell {
  # Pull in settings from TOML too (optional, but nice for menus)
  imports = [
    (perSystem.devshell.importTOML ./devshell.toml)
  ];

  # Handy env tweaks
  env = [
    { name = "RUST_BACKTRACE"; value = "1"; }
  ];

  # Quick commands will appear in `devshell` menu
  commands = [
    { name = "build";  help = "nix build package"; command = "nix build .#epub-indexer"; }
    { name = "run";    help = "run compiled bin";  command = "result/bin/epub-indexer --help || true"; }
    { name = "fmt";    help = "format all files";  command = "nix fmt"; }
    { name = "check";  help = "format check";      command = "nix flake check"; }
  ];

  # Tools you’ll want while developing the Rust app
  packages = with pkgs; [
    # Rust toolchain
    rustc cargo rustfmt clippy

    # useful utils
    git just ripgrep fd jq
    xxHash             # xxhsum for hashing
    pkg-config openssl # common needs for Rust deps
  ];
}

