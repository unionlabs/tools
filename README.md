# Union Development Tools

> [!NOTE]
> The project is under active development. Everything is subject to change.

### Launch `openvscode-server` editor

```md
Usage: launcher [flags]

Flags:
  --port=<port>  Port on which the vscode server will run     [default: 6699]
  --path=<path>  Repository to launch in vscode               [default: pwd]
  --host=<host>  Host on which the vscode server will run     [default: 127.0.0.1]
  --open         Launch vscode server in the default browser  [default: false]
  --help         Print this help message

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
