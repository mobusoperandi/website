{
  nixConfig.extra-experimental-features = [ "pipe-operators" ];

  inputs = {
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    FULLCALENDAR = {
      flake = false;
      url = "https://cdn.jsdelivr.net/npm/fullcalendar@6.0.2/index.global.min.js";
    };

    FULLCALENDAR_RRULE = {
      flake = false;
      url = "https://cdn.jsdelivr.net/npm/@fullcalendar/rrule@6.0.2/index.global.min.js";
    };

    INVERTICAT_LOGO = {
      flake = false;
      url = "https://raw.githubusercontent.com/primer/octicons/v19.0.0/icons/mark-github-16.svg";
    };

    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        parts.follows = "flake-parts";
        treefmt.follows = "treefmt-nix";
      };
    };

    nixpkgs.url = "nixpkgs/nixpkgs-unstable";

    RRULE = {
      flake = false;
      url = "https://cdn.jsdelivr.net/npm/rrule@2.7.2/dist/es5/rrule.min.js";
    };

    systems.url = "github:nix-systems/default";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    TWITTER_LOGO = {
      flake = false;
      url = "https://cdn.svgporn.com/logos/twitter.svg";
    };

    YOUTUBE_LOGO = {
      flake = false;
      url = "https://upload.wikimedia.org/wikipedia/commons/0/09/YouTube_full-color_icon_%282017%29.svg";
    };

    ZULIP_LOGO = {
      flake = false;
      url = "https://raw.githubusercontent.com/zulip/zulip/main/static/images/logo/zulip-icon-square.svg";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./crates/builder
        ./default.nix
        ./deploy-preview.nix
        ./devmode.nix
        ./fmt.nix
        ./nci.nix
        ./systems.nix
      ];
    };
}
