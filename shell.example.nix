let
  pkgs = import <nixpkgs> {};
  default = import ./default.nix {inherit pkgs;};
in
  default.overrideAttrs (attrs: {
    nativeBuildInputs = with pkgs;
      attrs.nativeBuildInputs
      ++ [
      ];
  })
