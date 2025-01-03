{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];

  perSystem.treefmt = {
    projectRootFile = "flake.nix";
    programs = {
      nixfmt.enable = true;
      prettier.enable = true;
      rustfmt.enable = true;
      shfmt.enable = true;
      toml-sort.enable = true;
    };
    settings = {
      on-unmatched = "fatal";
      formatter.shfmt.includes = [ "git_hooks/*" ];
      global.excludes = [
        ".editorconfig"
        "LICENSE"
      ];
    };
  };
}
