{
  description = "epubr — scan epubs and emit book.json";

  inputs = {
    # Nixpkgs 25.05 as requested
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    systems.url = "github:nix-systems/default";

    # Blueprint (latest; follows nixpkgs)
    blueprint.url = "github:numtide/blueprint";
    blueprint.inputs.nixpkgs.follows = "nixpkgs";
    blueprint.inputs.systems.follows = "systems";

    # Devshell (for nice shell UX + TOML support if you want it)
    devshell.url = "github:numtide/devshell";
    devshell.inputs.nixpkgs.follows = "nixpkgs";

    # naersk for Rust builds
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    # treefmt-nix (formatter entry point; gives alejandra & rustfmt)
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs:
    inputs.blueprint {
      inherit inputs;
      # <- IMPORTANT: we’re using the prefixed layout
      prefix = "nix/";
      # Let Blueprint pick systems from the input set (or override explicitly)
      systems = import inputs.systems;
    };
}
