{pkgs, ...}: {
  packages = with pkgs; [
    # Rust toolchain
    rustup

    # Code formatting tools
    treefmt
    alejandra
    rustfmt
    mdl

    # Rust dependency linting
    cargo-deny
  ];
}
