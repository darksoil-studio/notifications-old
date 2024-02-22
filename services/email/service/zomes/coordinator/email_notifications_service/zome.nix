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
      zomes.email_notifications_service = inputs.hcUtils.outputs.lib.rustZome {
					inherit excludedCrates;
          workspacePath = rootPath;
          holochain = inputs'.holochain;
					cratePath = ./.;
  		};
  	};
}
