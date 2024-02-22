{ inputs, rootPath, excludedCrates, ... }:

{
  perSystem =
    { inputs'
    , config
    , pkgs
    , system
    , lib
    , options
    , ...
    }: {
      zomes.email_notifications_service_integrity = inputs.hcUtils.outputs.lib.rustZome {
					inherit excludedCrates;
          workspacePath = rootPath;
          holochain = inputs'.holochain;
					cratePath = ./.;
      };
  	};
}
