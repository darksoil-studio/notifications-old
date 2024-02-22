{ inputs, ... }:

{
  imports = [];
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
        email_notifications_provider-dna = inputs.hcUtils.outputs.lib.dna {
          holochain = inputs'.holochain;
          dnaManifest = ./dna.yaml;
          zomes = {
            email_notifications_service_integrity = self'.packages.email-notifications-service-integrity-zome;
            email_notifications_service = self'.packages.email-notifications-service-coordinator-zome;
          };
        };
  		};
  	};
}
