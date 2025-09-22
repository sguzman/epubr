# nix/formatter.nix
# Must evaluate to a *derivation*, not a module/attrset.
# This uses treefmt-nix and *wraps* treefmt with the config.
{
  pkgs,
  inputs,
  system,
  ...
}: let
  # Evaluate a treefmt module (Nix-side config)
  tf = inputs.treefmt-nix.lib.evalModule pkgs {
    # Consider repo root
    projectRootFile = "flake.nix";

    # Enable Nix & Rust formatters
    programs.alejandra.enable = true; # <-- Alejandra installed via treefmt-nix
    programs.rustfmt.enable = true;

    # (Optional) keep these minimal; treefmt auto-discovers files
    settings.global.excludes = ["./result/*" "./target/*"];
  };
in
  # Return the wrapper derivation that `nix fmt` will run
  tf.config.build.wrapper
