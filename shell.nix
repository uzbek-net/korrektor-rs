{
  pkgs ? let
    lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
    nixpkgs = fetchTarball {
      url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
      sha256 = lock.narHash;
    };
  in
    import nixpkgs {overlays = [];},
  ...
}: let
  # Helpful nix function
  getLibFolder = pkg: "${pkg}/lib";
  mkLd = pkg: "-L${(getLibFolder pkg)}";

  # Manifest
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.stdenv.mkDerivation {
    name = "${manifest.name}-dev";

    # Compile time dependencies
    nativeBuildInputs = with pkgs; [
      # Hail the Nix
      nixd
      statix
      deadnix
      alejandra

      # Rust
      rustc
      cargo
      rustfmt
      clippy
      rust-analyzer
      cargo-watch

      # Autotools
      autoconf
      automake
      pkg-config

      # Other compile time dependencies
      pcre
      pcre2
      openssl
    ];

    # Runtime dependencies which will be in present
    # after activation
    buildInputs = with pkgs; [
      openssl
      # libressl
    ];

    # Set Environment Variables
    RUST_BACKTRACE = "full";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    # Compiler LD variables
    # > Make sure packages have /lib or /include path'es
    NIX_LDFLAGS = pkgs.lib.strings.concatStringsSep " " [
      (mkLd pkgs.pcre)
      (mkLd pkgs.pcre2)
    ];
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.libiconv
      pkgs.pcre
      pkgs.pcre2
    ];

    shellHook = ''
      # Fetch everything
      git fetch
    '';
  }
