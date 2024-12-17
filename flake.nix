{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs =
    inputs:
    inputs.utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          localSystem = system;
          overlays = [
            (final: prev: {
              naersk = prev.callPackage inputs.naersk { };

              alacritty-sdl2-term-rs = prev.callPackage (
                {
                  naersk,
                  pkg-config,
                  SDL2,
                  SDL2_ttf,
                }:

                naersk.buildPackage {
                  pname = "alacritty-sdl2-term-rs";
                  src = inputs.self;
                  nativeBuildInputs = [
                    pkg-config
                  ];
                  buildInputs = [
                    SDL2
                    SDL2_ttf
                  ];
                }
              ) { };
            })
          ];
        };
      in
      {
        packages = {
          default = inputs.self.packages."${system}".alacritty-sdl2-term-rs;
          alacritty-sdl2-term-rs = pkgs.alacritty-sdl2-term-rs;
        };

        devShells.default =
          with pkgs;
          mkShell {
            nativeBuildInputs = [
              rustfmt
              rustPackages.clippy
            ];
            inputsFrom = [ alacritty-sdl2-term-rs ];
          };
      }
    );
}
