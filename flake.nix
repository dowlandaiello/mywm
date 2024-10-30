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
          cargoHash = "sha256-MXvAqn+JwUdDcJBqioGxil4O200z7nYOznhPqtlK4Oc=";
          nativeBuildInputs = [ pkg-config makeWrapper ];
          buildInputs = [ libXinerama libX11 xmodmap ];
          libPath = lib.makeLibraryPath [ libXinerama libX11 ];
        };

        packages.default = packages.mywm;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ pkg-config makeWrapper ];
          buildInputs = [ libXinerama libX11 xmodmap ];
          libPath = lib.makeLibraryPath [ libXinerama libX11 ];
        };
      });
}

