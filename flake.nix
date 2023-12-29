{
  description = "MC-Oxide";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ]; # TODO: add more systems as they are supported
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit system; };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          shellHook = ''
            export CMAKE_GENERATOR=Ninja
          '';
          packages = with pkgs; [
            llvmPackages_latest.clang-unwrapped
            cmake
            ninja
            pkg-config

            cargo
            rustc

            qt6.qtbase
            qt6.qtwayland
          ];
        };
      });
    };
}