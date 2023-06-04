{pkgs ? import <nixpkgs> {}}: let
  pipe = pkgs.lib.trivial.pipe;
  toUpper = pkgs.lib.strings.toUpper;
  replaceStrings = builtins.replaceStrings;
  concatStringsSep = builtins.concatStringsSep;

  cargo-run-bin = pkgs.rustPlatform.buildRustPackage rec {
    pname = "cargo-run-bin";
    version = "0.4.1";
    src = pkgs.fetchCrate {
      inherit pname version;
      sha256 = "sha256-Muy21AuCpHCOO/XNds5Tn07d4mjhuyk1KzbCQdBT0bQ=";
    };
    cargoLock = {
      lockFile = src + "/Cargo.lock";
    };
    nativeBuildInputs = with pkgs; [
      cargo-llvm-cov
    ];
  };

  targetTriple = "x86_64-unknown-linux-gnu";
  envTargetTriple = pipe targetTriple [
    toUpper
    (replaceStrings ["-"] ["_"])
  ];
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      tokio-console
      rustup
      cargo-run-bin
      mob
      nodejs-18_x
      nodePackages.typescript-language-server
      nodePackages.typescript
    ];

    CARGO_BUILD_TARGET = targetTriple;
    "CARGO_TARGET_${envTargetTriple}_LINKER" = "${pkgs.clang}/bin/clang";
    RUSTFLAGS = concatStringsSep " " [
      "--codegen link-arg=-fuse-ld=${pkgs.mold}/bin/mold"
      "--cfg tokio_unstable"
    ];

    MOB_TIMER_ROOM = "mobusoperandi_website";
  }
