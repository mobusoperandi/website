{
  perSystem =
    { pkgs, ... }:
    {
      packages.deploy-preview = pkgs.writeShellApplication {
        name = "deploy-preview";
        runtimeInputs = [ pkgs.nodePackages.vercel ];
        text = ''
          vercel pull --yes --environment=preview --scope mobusoperandi --token="$VERCEL_TOKEN"
          URL=$(vercel deploy --prebuilt --token="$VERCEL_TOKEN")
          echo "URL=$URL" >> "$GITHUB_OUTPUT"
        '';
      };
    };
}
