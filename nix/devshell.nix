# nix/devshell.nix
{
  pkgs,
  perSystem,
  ...
}:
perSystem.devshell.mkShell {
  # Installed tools in the dev env
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
    alejandra
  ];

  # Nice banner + built-in menu
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

    {bold}Menu (devshell commands):{reset}
    $(
      if type -p menu >/dev/null; then
        menu
      else
        echo "  (menu unavailable)"
      fi
    )
  ''; # devshell.motd supports {bold}/{106}/{reset} styling. :contentReference[oaicite:1]{index=1}

  # Auto-enter fish for interactive shells
  devshell.interactive.fish.text = "exec ${pkgs.fish}/bin/fish"; # :contentReference[oaicite:2]{index=2}

  # Commands (these appear in the menu)
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
      help = "format Nix + Rust (Alejandra + rustfmt)";
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
      help = "Auto-fix with statix";
      command = "statix fix .";
    }
  ];
}
