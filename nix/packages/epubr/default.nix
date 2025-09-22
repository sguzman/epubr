# nix/packages/epubr/default.nix
{
  pkgs,
  inputs,
  system,
  ...
}: let
  naersk = inputs.naersk.lib."${system}";
in
  naersk.buildPackage {
    pname = "epubr";
    version = "0.1.0";
    src = ../../..; # repo root
    cargoBuildOptions = x: x ++ ["--locked"];
    nativeBuildInputs = with pkgs; [pkg-config];
    buildInputs = with pkgs; [openssl];
  }
