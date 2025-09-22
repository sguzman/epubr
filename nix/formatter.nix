{
  pkgs,
  inputs,
  ...
}: let
  biomeCmd = "${pkgs.biome}/bin/biome";
  shellhardenCmd = "${pkgs.shellharden}/bin/shellharden";
  fishIndentCmd = "${pkgs.fish}/bin/fish_indent";

  tf = inputs.treefmt-nix.lib.evalModule pkgs {
    projectRootFile = "flake.nix";

    programs = {
      alejandra.enable = true; # already had
      rustfmt.enable = true; # already had
      taplo.enable = true; # TOML
      stylua.enable = true; # Lua
    };

    settings = {
      global.excludes = [
        "./git/*"
        "./result/*"
        "./target/*"
      ];

      formatter = {
        biome = {
          command = biomeCmd;
          options = ["format" "--write"];
          includes = ["*.js" "*.cjs" "*.mjs" "*.jsx" "*.ts" "*.tsx" "*.html" "*.css"];
        };

        shellharden = {
          command = shellhardenCmd;
          options = ["-i"];
          includes = ["*.sh" "*.bash"];
        };

        fish-indent = {
          command = fishIndentCmd;
          options = ["--write"];
          includes = ["*.fish"];
        };
      };
    };
  };
in
  tf.config.build.wrapper
