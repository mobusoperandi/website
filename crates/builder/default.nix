{ inputs, lib, ... }:
{
  perSystem =
    { pkgs, ... }:
    {
      nci.crates.builder.drvConfig = {
        env = {
          inherit (inputs)
            FULLCALENDAR
            FULLCALENDAR_RRULE
            INVERTICAT_LOGO
            RRULE
            TWITTER_LOGO
            YOUTUBE_LOGO
            ZULIP_LOGO
            ;

          VOLLKORN = "${pkgs.vollkorn}/share/fonts/truetype/Vollkorn-Regular.ttf";
          TAILWINDCSS = lib.getExe pkgs.tailwindcss;
          CALENDAR_SNIPPET_JS = builtins.readFile ./snippet.js |> pkgs.writeText "snippet.js";

          TAILWINDCSS_CONFIG = pkgs.writeText "tailwind.config.js" ''
            module.exports = {
              plugins: [require("@tailwindcss/typography")],
            };
          '';

          TAILWINDCSS_INPUT = pkgs.writeText "input.css" ''
            @tailwind base;
            @tailwind components;
            @tailwind utilities;

            @layer base {
              a {
                @apply underline;
              }
            }
          '';
        };
      };

    };
}
