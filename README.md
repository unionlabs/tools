# Union Development Tools

> [!NOTE]
> The project is under active development. Everything is subject to change.

### Launch `openvscode-server` editor

```sh
export REPOSITORY_ABSOLUTE_PATH="/path/to/repository"

cargo run --package launcher -- --open
```

This flow is for early development. This will be packaged and used through nix. The end goal is:

```sh
nix run github:unionlabs/tools#code --open
```