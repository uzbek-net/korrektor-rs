{
  description = "Uzbek linguist utilities and functions for text processing.";

  inputs = {
    # Stable for keeping thins clean
    # nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";

    # Fresh and new for testing
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    # The flake-parts library
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} ({...}: {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem = {pkgs, ...}: {
        # Nix script formatter
        formatter = pkgs.alejandra;

        # Output package
        packages.default = pkgs.callPackage ./. {inherit pkgs;};

        # Development environment
        devShells.default = import ./shell.nix {inherit pkgs;};
      };
    });
}
