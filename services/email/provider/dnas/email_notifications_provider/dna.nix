{ inputs, ... }:

{
  # Import all ./zomes/coordinator/*/zome.nix and ./zomes/integrity/*/zome.nix  
  imports = (
      map (m: "${./.}/zomes/coordinator/${m}/zome.nix")
        (builtins.attrNames (builtins.readDir ./zomes/coordinator))
    )
    ++ 
    (
      map (m: "${./.}/zomes/integrity/${m}/zome.nix")
        (builtins.attrNames (builtins.readDir ./zomes/integrity))
    )
  ;
  perSystem =
    { inputs'
    , config
    , pkgs
    , system
    , lib
    , self'
    , options
    , ...
    }: {
  	  dnas.email_notifications_provider = inputs.hcUtils.outputs.lib.dna {
          holochain = inputs'.holochain;
          dnaManifest = ./dna/dna.yaml;
          zomes = config.zomes;
          # let
          #   coordinators = ;
          #   integrities = ;
          # in {
          #   inherit coordinators integrities;
          #   email_notifications_provider_integrity = self'.packages.email_notifications_provider-integrity-zome;
          #   email_notifications_provider = self'.packages.email_notifications_provider-coordinator-zome;
          # };
  		};
  	};
}
