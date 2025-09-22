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

  # inside perSystem.devshell.mkShell { ... }

  # Auto-enter fish *and* define a rich banner inside fish
  devshell.interactive.fish.text = ''
    function fish_greeting
      set_color -o cyan
      echo "epubr devshell"
      set_color normal

      echo
      set_color brwhite; echo "Toolchain:"; set_color normal
      echo "  • rustc         = "(rustc --version ^/dev/null | cut -d' ' -f2-)
      echo "  • cargo         = "(cargo --version ^/dev/null | cut -d' ' -f2-)
      echo "  • rustfmt       = "(rustfmt --version ^/dev/null | cut -d' ' -f2-)
      echo "  • rust-analyzer = "(rust-analyzer --version ^/dev/null)
      echo "  • alejandra     = "(alejandra --version ^/dev/null)
      echo "  • statix        = "(statix --version ^/dev/null)
      echo "  • deadnix       = "(deadnix --version ^/dev/null)

      echo
      set_color brwhite; echo "Project shortcuts:"; set_color normal
      echo "  • build    → nix build .#epubr"
      echo "  • run      → cargo run --"
      echo "  • fmt      → nix fmt"
      echo "  • check    → cargo check --all-targets"
      echo "  • test     → cargo test"
      echo "  • lint:nix → statix check . && deadnix ."
      echo "  • fix:nix  → statix fix ."

      echo
      set_color brwhite; echo "Menu (devshell commands):"; set_color normal
      if type -q menu
        menu
      else
        echo "  (menu unavailable)"
      end

      echo
    end

    # finally hop into fish
    exec ${pkgs.fish}/bin/fish
  '';

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
