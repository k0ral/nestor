{
  inputs = {
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/574d1eac1c200690e27b8eb4e24887f8df7ac27c";
  };

  outputs = { self, nixpkgs, naersk }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      naersk-lib = pkgs.callPackage naersk { };
    in {
      defaultPackage.${system} = naersk-lib.buildPackage ./.;
      devShell.${system} = with pkgs; mkShell {
        buildInputs = [ cargo cargo-watch rustc rustfmt pre-commit rustPackages.clippy ];
        RUST_SRC_PATH = rustPlatform.rustLibSrc;
      };
    };
}
