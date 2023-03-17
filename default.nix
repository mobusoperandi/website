{pkgs ? import <nixpkgs> {}}: let
  pipe = pkgs.lib.trivial.pipe;
  toUpper = pkgs.lib.strings.toUpper;
  replaceStrings = builtins.replaceStrings;

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

  targetTriple = "x86_64-unknown-linux-gnu";
  envTargetTriple = pipe targetTriple [
    toUpper
    (replaceStrings ["-"] ["_"])
  ];
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      rustup
      cargo-run-bin
      mob
      nodejs-19_x
      nodePackages.typescript-language-server
      nodePackages.typescript
    ];

    CARGO_BUILD_TARGET = targetTriple;
    "CARGO_TARGET_${envTargetTriple}_LINKER" = "${pkgs.clang}/bin/clang";
    RUSTFLAGS = "--codegen link-arg=-fuse-ld=${pkgs.mold}/bin/mold";

    MOB_TIMER_ROOM = "mobusoperandi_website";
  }
