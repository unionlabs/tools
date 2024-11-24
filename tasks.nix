{
  fmt.exec = ''
    taplo fmt *.toml
    nixfmt *.nix --width=100
    biome format . --write
    cargo fmt --all -- --config-path=rustfmt.toml
  '';
  lint.exec = ''
    taplo lint *.toml
    biome lint . --write
    cargo clippy --all-targets --all-features -- -A clippy::pedantic
    deadnix --no-lambda-pattern-names && statix check .
  '';
  # options:
  # $WORKSPACE
  build.exec = ''
    cargo build --release "$@"
  '';
  build-all.exec = ''
    cargo build --release --all
  '';
  # options:
  # $WORKSPACE
  dev.exec = ''
    cargo run --package "$@"
  '';
  rustdoc.exec = ''
    cargo rustdoc -- --default-theme='ayu'
  '';
  clean.exec = ''
    rm -rf build
    rm -rf target
  '';
}
