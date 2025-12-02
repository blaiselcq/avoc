{ }:
let pkgs = import <nixpkgs> { };
in pkgs.mkShell {
  name = "avc";

  buildInputs = with pkgs; [
    cargo
    clippy
    docopts
    rust-analyzer
    rustc
    rustfmt
    hyperfine
  ];
}

