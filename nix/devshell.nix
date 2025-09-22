# nix/devshell.nix
{
  pkgs,
  perSystem,
  ...
}: let
  # Single source of truth for tools in the shell
  toolPkgs = with pkgs; [
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

  # Render a clean "name version" line for each tool from Nix metadata.
  # Falls back if `pname`/`version` are missing.
  fmtPkg = p: let
    hasPN = p ? pname;
    pn =
      if hasPN
      then p.pname
      else (p.name or "pkg");
    ver =
      if p ? version
      then p.version
      else "";
    line =
      if ver == ""
      then "• ${pn}"
      else "• ${pn} ${ver}";
  in
    "        " + line;

  packagesSummary =
    builtins.concatStringsSep "\n" (map fmtPkg toolPkgs);

  # Banner script for fish; prints dynamic packages + the devshell menu
  banner = pkgs.writeText "epubr-banner.fish" ''
        function fish_greeting
          set_color -o cyan
          echo "epubr devshell"
          set_color normal

          # Where & who
          set_color brwhite; echo ""; echo "Project:"; set_color normal
          echo "  • PWD     → "(pwd)
          if command -q git
            echo "  • branch  → "(git rev-parse --abbrev-ref HEAD ^/dev/null)
          end

          # Packages come from Nix metadata (auto-updates when you change toolPkgs)
          set_color brwhite; echo ""; echo "Packages (from Nix):"; set_color normal
    ${packagesSummary}

          # Shortcuts = devshell commands; `menu` is dynamic
          set_color brwhite; echo ""; echo "Menu (devshell commands):"; set_color normal
          if type -q menu
            menu
          else
            echo "  (menu unavailable)"
          end

          echo ""
          set_color brwhite; echo "Tip:"; set_color normal
          echo "  • Run 'devhelp' anytime to reprint this banner."
          echo ""
        end

        # Reprint on demand
        function devhelp
          fish_greeting
        end
  '';
in
  perSystem.devshell.mkShell {
    # Use the same list for the actual devshell packages (stays in sync)
    packages = toolPkgs;

    # Auto-enter fish and source the banner script before the prompt appears
    devshell.interactive.fish.text = "exec ${pkgs.fish}/bin/fish -C 'source ${banner}'";

    # Keep devshell’s own MOTD quiet; fish prints our banner
    motd = "";

    # Commands (menu is auto-generated from here)
    commands = [
      {
        name = "devhelp";
        help = "reprint this banner/help";
        command = "${pkgs.fish}/bin/fish -c devhelp";
      }

      {
        name = "build";
        help = "nix build .#epubr";
        command = "nix build .#epubr";
      }
      {
        name = "run";
        help = "cargo run --";
        command = "cargo run --";
      }

      # formatters
      {
        name = "fmt";
        help = "format Nix + Rust (treefmt: alejandra+rustfmt)";
        command = "nix fmt";
      }
      {
        name = "fmt:nix";
        help = "format Nix only (Alejandra)";
        command = "alejandra .";
      }
      {
        name = "fmt:rust";
        help = "format Rust only (cargo fmt)";
        command = "cargo fmt --all";
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

      # linters
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
