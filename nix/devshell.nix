# nix/devshell.nix
# Uses numtide/devshell (nice menu, services, TOML option, etc.)
{
  pkgs,
  perSystem,
  inputs,
  ...
}:
perSystem.devshell.mkShell {
  # If you prefer plain mkShell, see the alt version below
  # (and you could remove the devshell input).

  # Basic Rust dev deps (25.05 channel)
  packages = with pkgs; [
    cargo
    rustc
    rustfmt
    rust-analyzer
    pkg-config
    openssl
    jq
  ];

  # Useful commands in the shell menu
  commands = [
    {
      name = "build";
      command = "nix build .#epubr";
    }
    {
      name = "check";
      command = "cargo check --all-targets";
    }
    {
      name = "test";
      command = "cargo test";
    }
    {
      name = "fmt";
      command = "nix fmt";
    }
    {
      name = "run";
      command = "cargo run --";
    }
  ];

  env = [
    # Example: speed up Rust linkage & make pkg-config work for native deps
    {
      name = "RUSTFLAGS";
      value = "-C target-cpu=native";
    }
  ];
}
