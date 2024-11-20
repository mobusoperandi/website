{ inputs, ... }:
{
  imports = [
    inputs.nci.flakeModule
    inputs.devshell.flakeModule
  ];

  perSystem =
    { pkgs, ... }:
    {
      nci.projects.default = {
        numtideDevshell = "default";
        path = ./.;
      };

      devshells.default = {
        devshell.packages = [
          pkgs.rust-analyzer
          pkgs.typescript
        ];
      };
    };
}
