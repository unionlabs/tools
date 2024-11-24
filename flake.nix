{
  description = "Union Labs Development Tools";
  inputs = {

    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
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
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
      });
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
                enterShell = '''';

                # https://devenv.sh/reference/options/
                scripts = import ./tasks.nix;

                languages.nix.enable = true;
                languages.rust = {
                  enable = true;
                  channel = "nightly";
                  targets = [ ];
                  components = [
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
