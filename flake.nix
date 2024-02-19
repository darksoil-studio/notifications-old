{
  description = "Template for Holochain app development";
  
  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";

    versions.url = "github:holochain/holochain?dir=versions/weekly";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };

    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs @ { ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain.devShells;
        perSystem =
          { inputs'
          , config
          , pkgs
          , system
          , lib
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs'.holochain.devShells.holonix ];
              packages = with pkgs; [
                nodejs-18_x
                # more packages go here
                cargo-nextest
                binaryen
              ];
            };

            packages = {
              email-notifications-provider =
                let 
                  craneLib = inputs.crane.lib.${system};
                in
                  craneLib.buildPackage {
                    src = ./.;
                    cargoExtraArgs = "-p email_notifications_provider_runner";
                    pname = "email_notifications_provider_runner";
                    
                    buildInputs = (with pkgs; [ openssl sqlcipher ])
                      ++ (lib.optionals pkgs.stdenv.isDarwin
                      (with pkgs.darwin.apple_sdk_11_0.frameworks; [
                        AppKit
                        CoreFoundation
                        CoreServices
                        Security
                        IOKit
                      ]));

                    nativeBuildInputs = (with pkgs; [ makeWrapper perl pkg-config go ])
                      ++ lib.optionals pkgs.stdenv.isDarwin
                      (with pkgs; [ xcbuild libiconv ]);
                    # nativeBuildInputs = [ pkgs.openssl.dev pkgs.pkg-config ];
                    # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
                  };
            };
          };
      };
}
