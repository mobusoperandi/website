{pkgs ? import <nixpkgs> {}}: let
  cargo-run-bin = pkgs.rustPlatform.buildRustPackage rec {
    pname = "cargo-run-bin";
    version = "0.3.1";
    src = pkgs.fetchCrate {
      inherit pname version;
      sha256 = "sha256-4zu3vXlGKmBlB5we3qR0V6LIALqPxVGosBF4EkO7aHE=";
    };
    cargoLock = {
      lockFile = src + "/Cargo.lock";
    };
  };
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      openssl
      pkg-config
    ];
    nativeBuildInputs = with pkgs; [
      rustup
      cargo-run-bin
      mob
      nodejs-19_x
      nodePackages.typescript-language-server
      nodePackages.typescript
    ];
    MOB_TIMER_ROOM = "mobusoperandi_website";
  }
