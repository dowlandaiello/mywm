{
  description = "Flake packaging my window manager";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in with pkgs;
      with xorg; rec {
        packages.mywm = rustPlatform.buildRustPackage {
          pname = "mywm";
          version = "0.1.0";
          src = ./.;
          cargoHash = lib.fakeHash;
          nativeBuildInputs = [ pkg-config makeWrapper ];
          buildInputs = [ libXinerama libX11 ];
          libPath = lib.makeLibraryPath [ libXinerama libX11 ];
        };

        packages.default = packages.mywm;
      });
}

