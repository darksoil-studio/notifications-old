{ inputs, rootPath, excludedCrates, ... }:

{
  perSystem =
    { inputs'
    , config
    , pkgs
    , system
    , lib
    , ...
    }: {
  	  packages.email_notifications_provider_coordinator = inputs.hcUtils.outputs.lib.rustZome {
				inherit excludedCrates;
        workspacePath = rootPath;
        holochain = inputs'.holochain;
				cratePath = ./.;
  		};
  	};
}
