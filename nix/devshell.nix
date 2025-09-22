{
  pkgs,
  perSystem,
  ...
}:
perSystem.devshell.mkShell {
  # Tools available inside the shell
  packages = with pkgs; [
    fish
    cargo
    rustc
    rustfmt
    rust-analyzer
    pkg-config
    openssl
    jq
    statix
    deadnix
    alejandra # so `nix fmt` has it in PATH too
  ];

  # Pretty banner with versions + command menu
  motd = ''
    {bold}{106}epubr devshell{reset}

    {bold}Toolchain:{reset}
      • rustc         = $(rustc --version 2>/dev/null | cut -d' ' -f2-)
      • cargo         = $(cargo --version 2>/dev/null | cut -d' ' -f2-)
      • rustfmt       = $(rustfmt --version 2>/dev/null | cut -d' ' -f2-)
      • rust-analyzer = $(rust-analyzer --version 2>/dev/null || true)
      • alejandra     = $(alejandra --version 2>/dev/null || true)
      • statix        = $(statix --version 2>/dev/null || true)
      • deadnix       = $(deadnix --version 2>/dev/null || true)

    {bold}Project shortcuts:{reset}
      • build         → nix build .#epubr
      • run           → cargo run --
      • fmt           → nix fmt
      • check         → cargo check --all-targets
      • test          → cargo test
      • lint:nix      → statix check . && deadnix .
      • fix:nix       → statix fix .

    {bold}Menu (from devshell):{reset}
    $(
      if type -p menu >/dev/null; then
        menu
      else
        echo "  (menu unavailable)"
      fi
    )
  '';

  # Define the same commands so they appear in the devshell menu
  commands = [
    {
      name = "build";
      help = "nix build .#epubr";
      command = "nix build .#epubr";
    }
    {
      name = "run";
      help = "cargo run -- …";
      command = "cargo run --";
    }
    {
      name = "fmt";
      help = "format Nix + Rust (treefmt/Alejandra+rustfmt)";
      command = "nix fmt";
    }
    {
      name = "check";
      help = "cargo check (all targets)";
      command = "cargo check --all-targets";
    }
    {
      name = "test";
      help = "cargo test";
      command = "cargo test";
    }
    {
      name = "lint:nix";
      help = "Nix lint: statix + deadnix";
      command = "statix check . && deadnix .";
    }
    {
      name = "fix:nix";
      help = "Auto-fix with statix (safe-ish)";
      command = "statix fix .";
    }
  ];

  # Auto-enter fish
  shellHook = ''exec ${pkgs.fish}/bin/fish'';
}
