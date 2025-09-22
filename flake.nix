{
  description = "epub-indexer (Rust) with Blueprint, devshell, naersk, treefmt";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-25.05";

    # Blueprint: discovers files in nix/ and wires outputs
    blueprint.url = "github:numtide/blueprint";
    blueprint.inputs.nixpkgs.follows = "nixpkgs";

    # Devshell (menu, services, etc.)
    devshell.url = "github:numtide/devshell";
    devshell.inputs.nixpkgs.follows = "nixpkgs";

    # Build Rust
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    # Formatter orchestrator (makes `nix fmt` run alejandra + rustfmt)
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";

    systems.url = "github:nix-systems/default"; # default set of systems
  };

  outputs = inputs:
    inputs.blueprint {
      inherit inputs;
      prefix = "nix/";         # look under nix/ for devshell, packages, formatter, etc.
      systems = inputs.systems; # optional explicit systems (good default)
      # You can also set nixpkgs.config.allowUnfree = true; here if needed
    };
}

