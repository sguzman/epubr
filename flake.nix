{
  description = "epubr — EPUB indexer (Rust) using Blueprint";

  inputs = {
    # Stable
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-25.05";

    # Blueprint (keeps flake tiny; maps folders->outputs)
    blueprint.url = "github:numtide/blueprint";
    blueprint.inputs.nixpkgs.follows = "nixpkgs";

    # Rust build
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    # Formatter orchestrator: makes `nix fmt` work (Alejandra + rustfmt)
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";

    # Common systems list
    systems.url = "github:nix-systems/default";
  };

  # Hand control to Blueprint; scan ./nix for modules
  outputs = inputs:
    inputs.blueprint {
      inherit inputs;
      prefix = "nix/"; # look under ./nix
      systems = inputs.systems; # multi-system ready
    };
}
