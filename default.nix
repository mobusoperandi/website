{
  perSystem =
    { pkgs, config, ... }:
    {
      packages.default =
        pkgs.runCommand "build"
          {
            nativeBuildInputs = [
              config.nci.outputs.builder.packages.dev
              pkgs.git
            ];
          }
          ''
            builder ${./mobs} $out
          '';
    };
}
