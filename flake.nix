{
  description = "MC-Oxide";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.complete;
    in {
      devShells = {
        default = pkgs.mkShell {
          CMAKE_GENERATOR = "Ninja";

          nativeBuildInputs = with pkgs; [
            clang-tools
            cmake
            ninja
            pkg-config
            (toolchain.withComponents [
              "cargo"
              "rustc"
              "rust-src"
              "rustfmt"
              "clippy"
            ])
          ];

          buildInputs = with pkgs; [
            qt6.qtbase
            qt6.qtwayland
          ];
        };
      };
    });
}
