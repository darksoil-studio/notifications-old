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
      packages.email_notifications_service_coordinator = inputs.hcUtils.outputs.lib.rustZome {
				inherit excludedCrates;
        workspacePath = rootPath;
        holochain = inputs'.holochain;
				cratePath = ./.;
  		};
  	};
}
