{ inputs, lib, ... }:
{
  imports = [
    inputs.nci.flakeModule
    inputs.devshell.flakeModule
  ];

  perSystem =
    { config, pkgs, ... }:
    {
      nci.projects.default = {
        numtideDevshell = "default";
        path = ./.;
        useClippy = true;
      };

      devshells.default = {
        devshell.packages = [
          pkgs.rust-analyzer
          pkgs.typescript
        ];
        env = lib.concatMapAttrs (_: crate: crate.drvConfig.env) config.nci.crates |> lib.attrsToList;
      };
    };
}
