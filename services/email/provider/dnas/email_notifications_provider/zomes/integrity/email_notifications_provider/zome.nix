{ inputs, rootPath, excludedCrates, ... }:

{
  perSystem =
    { inputs'
    , config
    , pkgs
    , system
    , lib
    , self'
    , ...
    }: {
  	  packages.email_notifications_provider_integrity = inputs.hcUtils.outputs.lib.rustZome {
				inherit excludedCrates;
        workspacePath = rootPath;
        holochain = inputs'.holochain;
				cratePath = ./.;
			};
  	};
}
