{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };
      in rec {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # Rust toolchain
            cargo
            rustc

            # Code analysis tools
            clippy
            rust-analyzer

            # Code formatting tools
            treefmt
            alejandra
            rustfmt
            mdl

            # Rust dependency linting
            cargo-deny
          ];
        };
      }
    );
}
