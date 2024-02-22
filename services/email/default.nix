{ ... }:

{
  imports = [
    ./provider/app.nix
    ./provider/dnas/email_notifications_service/zomes/coordinator/email_notifications_bridge/zome.nix
    ./service/dna/dna.nix
    ./service/zomes/coordinator/email_notifications_service/zome.nix
    ./service/zomes/integrity/email_notifications_service/zome.nix
  ];
}
