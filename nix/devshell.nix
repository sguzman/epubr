# nix/devshell.nix
{
  pkgs,
  perSystem,
  ...
}:
perSystem.devshell.mkShell {
  motd = ""; # no orange banner
  packages = with pkgs; [
    fish
    cargo
    rustc
    rustfmt
    rust-analyzer
    pkg-config
    openssl
    jq
    statix
    deadnix # nix linters
  ];
  shellHook = ''exec ${pkgs.fish}/bin/fish'';
  commands = [
    {
      name = "build";
      command = "nix build .#epubr";
    }
    {
      name = "run";
      command = "cargo run --";
    }
    {
      name = "fmt";
      command = "nix fmt";
    }
    {
      name = "check";
      command = "cargo check --all-targets";
    }
    {
      name = "test";
      command = "cargo test";
    }
    {
      name = "lint:nix";
      command = "statix check . && deadnix .";
    }
    {
      name = "fix:nix";
      command = "statix fix .";
    }
  ];
}
