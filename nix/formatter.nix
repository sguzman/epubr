{inputs, ...}: {
  perSystem = {pkgs, ...}: {
    imports = [inputs.treefmt-nix.flakeModule];

    treefmt = {
      # Use treefmt as the flake formatter => `nix fmt` works
      flakeFormatter = true; # (default is true; being explicit)

      programs = {
        alejandra.enable = true; # formats *.nix (from nixpkgs)
        rustfmt.enable = true; # formats Rust
      };
    };
  };
}
