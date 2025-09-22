{
  pkgs,
  inputs,
  ...
}: let
  naersk = pkgs.callPackage inputs.naersk {};
in
  naersk.buildPackage {
    pname = "epubr";
    src = ../../..; # repo root (where Cargo.toml lives)
    # Enable when you add tests:
    # doCheck = true;
  }
