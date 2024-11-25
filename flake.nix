{
  description = "Union Labs Development Tools";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";

    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      devenv,
      nixpkgs,
      systems,
      ...
    }@inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          devenv-up = self.devShells.${system}.default.config.procfileScript;
          packages.${system}.devenv-test = self.devShells.${system}.default.config.test;
          ucode = pkgs.writeShellApplication {
            name = "ucode";
            runtimeInputs = [ pkgs.openvscode-server ];
            text = ''
              openvscode-server --update-extensions --disable-telemetry --disable-telemetry --accept-server-license-terms --start-server "$@"
            '';
          };
        }
      );
      devShells = forEachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {

          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              {
                enterShell = ''
                  echo ""
                '';

                # https://devenv.sh/reference/options/
                scripts = import ./tasks.nix;

                languages.nix.enable = true;
                languages.rust = {
                  enable = true;
                  channel = "nightly";
                  targets = [ ];
                  components = [
                    "miri"
                    "rustc"
                    "cargo"
                    "clippy"
                    "rustfmt"
                    "rust-analyzer"

                  ];
                };

                env.ZKGM = "zkgm";
                packages = with pkgs; [
                  jq
                  git
                  nixd
                  taplo
                  biome
                  typos
                  procs
                  direnv
                  deadnix
                  nixfmt-rfc-style
                  # Node.js for taplo
                  nodePackages_latest.nodejs
                ];
              }
            ];
          };
        }
      );
    };

  nixConfig = {
    extra-substituters = "https://devenv.cachix.org";
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
  };
}
