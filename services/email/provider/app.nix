{ inputs, ... }:

{
  imports = [
		./dnas/email_notifications_provider/dna.nix
	];
  perSystem =
    { inputs'
    , config
    , pkgs
    , system
    , lib
		, self'
		# , zomes
		, options
    , ...
    }: {
  	  packages = {
        email_notifications_provider-app = 
			  let
		      email_notifications_service-dna = inputs.hcUtils.outputs.lib.dna {
		        holochain = inputs'.holochain;
		        dnaManifest = ./dnas/email_notifications_service/dna/dna.yaml;
						zomes = {
							email_notifications_service_coordinator = self'.packages.email_notifications_service_coordinator;
							email_notifications_service_integrity = self'.packages.email_notifications_service_integrity;
							email_notifications_bridge = self'.packages.email_notifications_bridge;
						};
		      };
				in
				inputs.hcUtils.outputs.lib.happ {
          holochain = inputs'.holochain;
          happManifest = ./app/happ.yaml;
					dnas = {
						email_notifications_service = email_notifications_service-dna;
						email_notifications_provider = self'.packages.email_notifications_provider;
					};
        };
  		};
  	};
}
