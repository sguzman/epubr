# nix/formatter.nix
{inputs, ...}: {
  # Module-style integration: import treefmt-nix's flakeModule
  perSystem = {pkgs, ...}: {
    imports = [inputs.treefmt-nix.flakeModule];

    treefmt = {
      # Wire up flake outputs automatically
      flakeFormatter = true; # `nix fmt`
      flakeCheck = true; # `nix flake check` includes formatting check

      # Built-in formatters
      programs = {
        alejandra.enable = true; # *.nix
        rustfmt.enable = true; # Rust
        taplo.enable = true; # TOML
        stylua.enable = true; # Lua
      };

      # Extra formatters (custom)
      settings = {
        global.excludes = ["./result/*" "./target/*"];

        formatter = {
          # Biome for JS/TS/TSX/JSX/HTML/CSS
          biome = {
            command = "${pkgs.biome}/bin/biome";
            options = ["format" "--write"];
            includes = [
              "*.js"
              "*.cjs"
              "*.mjs"
              "*.jsx"
              "*.ts"
              "*.tsx"
              "*.html"
              "*.css"
            ];
          };

          # shellharden (in-place rewrite) for shell scripts
          shellharden = {
            command = "${pkgs.shellharden}/bin/shellharden";
            options = ["-i"];
            includes = ["*.sh" "*.bash"];
          };

          # fish formatter
          "fish-indent" = {
            command = "${pkgs.fish}/bin/fish_indent";
            options = ["--write"];
            includes = ["*.fish"];
          };
        };
      };
    };
  };
}
