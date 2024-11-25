{
  pkgs,
  inputs,
  ...
}:
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
