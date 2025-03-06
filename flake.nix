{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # 👇 we have a new input!
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };
  #      it's destructured here 👇
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    # calling a function from `flake-utils` that takes a lambda
    # that takes the system we're targetting
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
          ./rust-toolchain.toml;
        nativeBuildInputs = with pkgs; [ rustToolchain ];
        buildInputs = with pkgs; [ libpq ];
      in with pkgs; {
        devShells.default = mkShell { inherit buildInputs nativeBuildInputs; };
      });
}
