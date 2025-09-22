{ pkgs, inputs, ... }:
{
  # Integrate treefmt-nix module and define formatters
  perSystem = { config, self', pkgs, ... }: {
    imports = [ inputs.treefmt-nix.flakeModule ];

    treefmt = {
      # Enable as flake formatter => `nix fmt` works
      flakeFormatter = true;
      flakeCheck = true;              # adds a check to `nix flake check`

      programs = {
        alejandra.enable = true;      # format *.nix
        rustfmt.enable = true;        # format Rust
      };
    };
  };
}

