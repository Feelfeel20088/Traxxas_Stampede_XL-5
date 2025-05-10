{
  description = "Converts an entire directory to a pdf used just once to convert my entire program to a pdf for my AP";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
        let
            pkgs = import nixpkgs {
            inherit system;
        };
        in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "rc_car_controller";
            version = "0.1.1";

            src = ./.;

            nativeBuildInputs = [ pkgs.cargo ];
            buildInputs = [ pkgs.SDL2 ];

            cargoLock = {
                lockFile = ./Cargo.lock;
            };

            meta = with pkgs.lib; {
              description = "RC car controlled by a steamdeck or any compatiple SDL2 controller";
              license = licenses.mit;
              platforms = platforms.all;
            };

        };


      });
}