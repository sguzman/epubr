{ pkgs, perSystem, ... }:

perSystem.devshell.mkShell {
  packages = with pkgs; [
    fish
    cargo rustc rustfmt rust-analyzer
    pkg-config openssl jq
    statix deadnix
    alejandra
  ];

  # -------- Banner: define once as a fish script, then source it on startup --------
  # devshell scripts are POSIX-parsed before your shell starts, so we must NOT put fish
  # syntax directly here. Instead, write a fish file and source it via `fish -C`.
  let
    banner = pkgs.writeText "epubr-banner.fish" ''
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
    '';
  in

  # Auto-enter fish and source the banner script BEFORE the prompt is shown
  devshell.interactive.fish.text = "exec ${pkgs.fish}/bin/fish -C 'source ${banner}'";

  # Devshell menu commands (what `menu` prints)
  commands = [
    { name = "build";    help = "nix build .#epubr";                        command = "nix build .#epubr"; }
    { name = "run";      help = "cargo run -- …";                           command = "cargo run --"; }
    { name = "fmt";      help = "format Nix + Rust (Alejandra + rustfmt)";  command = "nix fmt"; }
    { name = "check";    help = "cargo check (all targets)";                command = "cargo check --all-targets"; }
    { name = "test";     help = "cargo test";                               command = "cargo test"; }
    { name = "lint:nix"; help = "Nix lint: statix + deadnix";               command = "statix check . && deadnix ."; }
    { name = "fix:nix";  help = "Auto-fix with statix";                     command = "statix fix ."; }
  ];
}

