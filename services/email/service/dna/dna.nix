{ inputs, ... }:

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
  	  packages = {
        email_notifications_service-dna = inputs.hcUtils.outputs.lib.dna {
          holochain = inputs'.holochain;
          dnaManifest = ./dna.yaml;
          zomes = {
            email_notifications_service_integrity = self'.packages.email_notifications_service-integrity-zome;
            email_notifications_service = self'.packages.email_notifications_service-coordinator-zome;
          };
        };
  		};
  	};
}
