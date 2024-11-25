# Union Development Tools

> [!NOTE]
> The project is under active development. Everything is subject to change.

### Launch `openvscode-server` editor

```md
Usage: launcher [OPTIONS]

Options:
    --port  <VALUE>  Port to run the server on [default: 6699]
    --host  <VALUE>  Host to run the server on [default: 127.0.0.1]
    --path  <VALUE>  Repository to launch in VSCode [default: .]
    --open          Open the server in the browser
-h, --help          Print help
-V, --version       Print version

Example:
  launcher --path=/path/to/repo --port=6699 --host=localhost --open
```

For development:

```sh
cargo run --package=launcher --quiet -- \
  --path="$(pwd)" \
  --port=6699 \
  --host=127.0.0.1 \
  --open
```

___

This flow is for early development. This will be packaged and used through nix. The end goal is the following with he same flags as above:

```sh
nix run github:unionlabs/tools#code --open
```
