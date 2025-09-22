{pkgs}:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    git
    ripgrep
    fd
    jq
    xxHash # xxhsum
    pkg-config
    openssl
  ];

  env = {
    RUST_BACKTRACE = "1";
  };

  shellHook = ''
    echo "[epubr] devshell ready: rustc $(rustc --version | cut -d' ' -f2-)"
  '';
}
