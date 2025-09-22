{ pkgs, perSystem, flake, system, inputs, pname ? "epubr", ... }:
let
  naersk = pkgs.callPackage inputs.naersk { };
in
naersk.buildPackage {
  inherit pname;
  src = flake;    # your repo root (expects Cargo.toml & Cargo.lock there)
  # If your Cargo files are not at the repo root, instead do:
  #   root = ./../../..;  # adjust to where Cargo.toml lives
  #   src = flake;
  doCheck = false;    # turn on later (with tests) or use cargoTestCommands
}

