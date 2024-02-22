{
  description = "Template for Holochain app development";
  
  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";

    versions.url = "github:holochain/holochain?dir=versions/weekly";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };
		hcUtils.url = "github:holochain-open-dev/common";
  };

  outputs = inputs @ { ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
        specialArgs.rootPath = ./.;
        specialArgs.excludedCrates = ["email_notifications_provider_runner"];
      }
      {
        imports = [
          ./services/email/default.nix
        ];

        systems = builtins.attrNames inputs.holochain.devShells;
        perSystem =
          { inputs'
          , config
          , pkgs
          , system
          , lib
          , self'
          , ...
          }: {

            # config.packages = config.zomes // config.dnas;
            # config.zomes = {};
            # packages.zomes = lib.makeScope pkgs.newScope (_: {});

            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs'.holochain.devShells.holonix ];
              packages = with pkgs; [
                nodejs-18_x
                # more packages go here
                cargo-nextest
              ];
            };
          };
      };
}
