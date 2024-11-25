> [!NOTE]
> The project is under active development. Everything is subject to change.

# Union Development Tools

Launch Open VSCode with all the tools and extensions you need to ship.

```sh
nix run github:unionlabs/tools#ucode
```

pass `--help` flag to see all the options

```sh
nix run github:unionlabs/tools#ucode -- --help
```

```md
Quintessential development environment - powered by Open VSCode & VSX

Usage: launcher [OPTIONS]

Options:
    --port  <VALUE>  Port to run the server on [default: 6699]
    --host  <VALUE>  Host to run the server on [default: 127.0.0.1]
    --path  <VALUE>  Repository to launch in VSCode [default: .]
    --open           Open the server in the browser
-h, --help           Print help
-V, --version        Print version

```

### Passing arguments to `ucode`

```sh
nix run github:unionlabs/tools#ucode -- \
  --path='/absolute/path/to/repo' \
  --port='6699' \
  --host='127.0.0.1' \
  --open
```

### Development

```sh
cargo run --package=launcher --quiet -- \
  --path="$(pwd)" \
  --port=6699 \
  --host=127.0.0.1 \
  --open
```
