{
  perSystem =
    { pkgs, ... }:
    {
      devshells.default = {
        devshell.packages = [
          pkgs.coreutils
          pkgs.git
        ];
        commands = [
          {
            name = "devmode";
            command = ''
              cargo run \
                $(git rev-parse --show-toplevel)/mobs \
                $(mktemp --directory) \
                --open
            '';
          }
        ];
      };
    };
}
