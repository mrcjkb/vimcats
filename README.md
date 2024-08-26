<h1 align="center">:cat2: vimCATS :book:</h1>
<p align="center"><sup>A CLI to generate vimdoc from LuaCATS. Forked from lemmy-help.</sup></p>

<!-- TODO: Update gif -->
![vimcats](https://user-images.githubusercontent.com/24727447/164423469-b26fea39-2ef7-497c-8156-5a4c01bc30f8.gif "Generating help docs")

### What?

`vimcats` is a LuaCATS parser as well as a CLI
which takes that parsed tree and converts it into vim help docs.

### Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/vimcats.svg)](https://repology.org/project/vimcats/versions)

- Using `cargo`

```bash
cargo install vimcats --features=cli
```

### LuaCATS

To properly generate docs you should follow luaCATS spec.
The parser is capable of parsing most (not all) of the LuaCATS syntax.
You can read the following doc which can give you the idea on how to
properly write LuaCATS annotations.

- [Writing LuaCATS annotations](./luaCATS.md)

### Usage

Using the CLI is simple just give it the path to the lua files;
it will parse them and prints help doc to `stdout`.

```bash
vimcats /path/to/{first,second,third}.lua > doc/PLUGIN_NAME.txt
```

### CLI

```text
vimcats

USAGE:
    vimcats [FLAGS] [OPTIONS] <FILES>...

ARGS:
    <FILES>...                  Path to lua files

FLAGS:
    -h, --help                  Print help information
    -v, --version               Print version information
    -M, --no-modeline           Don't print modeline at the end
    -f, --prefix-func           Prefix function name with ---@mod name
    -a, --prefix-alias          Prefix ---@alias tag with return/---@mod name
    -c, --prefix-class          Prefix ---@class tag with return/---@mod name
    -t, --prefix-type           Prefix ---@type tag with ---@mod name
        --expand-opt            Expand '?' (optional) to 'nil' type

OPTIONS:
    -i, --indent <u8>           Controls the indent width [default: 4]
    -l, --layout <layout>       Vimdoc text layout [default: 'default']
                                - "default" : Default layout
                                - "compact[:n=0]" : Aligns [desc] with <type>
                                  and uses {n}, if provided, to indent the
                                  following new lines. This option only
                                  affects ---@field and ---@param tags
                                - "mini[:n=0]" : Aligns [desc] from the start
                                  and uses {n}, if provided, to indent the
                                  following new lines. This option affects
                                  ---@field, ---@param and ---@return tags

USAGE:
    vimcats /path/to/first.lua /path/to/second.lua > doc/PLUGIN_NAME.txt
    vimcats -c -a /path/to/{first,second,third}.lua > doc/PLUGIN_NAME.txt
    vimcats --layout compact:2 /path/to/plugin.lua > doc/PLUGIN_NAME.txt

NOTES:
    - The order of parsing + rendering is relative to the given files
```

### CI

```yaml
name: vimcats

on: [push]

env:
  PLUGIN_NAME: plugin-name

jobs:
  docs:
    runs-on: ubuntu-latest
    name: luaCATS to vimdoc
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@master

      - name: Install vimcats
        run: cargo install vimcats --features=cli

      - name: Generate docs
        run: vimcats [args] <path> > doc/${{env.PLUGIN_NAME}}.txt

      - name: Commit
        uses: stefanzweifel/git-auto-commit-action@v4
        with:
          branch: ${{ github.head_ref }}
          commit_message: "chore(docs): auto-generate vimdoc"
          file_pattern: doc/*.txt
```

### Nix flake

This project provides a flake so you can use it with frameworks
like [git-hooks.nix](https://github.com/cachix/git-hooks.nix).

Here is basic example:

> [!NOTE]
>
> You will likely want to ajust the flake to be used
> with multiple systems.

```nix
{ 
  inputs = {
    git-hooks.url = "github:cachix/git-hooks.nix";
    vimcats.url = "github:mrcjkb/vimcats";
  };
  
  outputs = {
    self,
    nixpkgs,
    git-hooks,
    vimcats,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    docgen = final.writeShellApplication {
      name = "docgen";
      runtimeInputs = [
        inputs.vimcats.packages.${final.system}.default
      ];
      text = ''
        mkdir -p doc
        vimcats [args] <path> > doc/<plugin-name>.txt
      '';
    };
    git-hooks-check = git-hooks.lib.${system}.run {
      src = self;
      hooks = {
        docgen = {
          enable = true;
          name = "docgen";
          entry = "${pkgs.docgen}/bin/docgen";
          files = "\\.(lua)$";
          pass_filenames = false;
        };
      };
    };
  in {
    devShells.${system}.default = {
      pkgs.mkShell {
        buildInputs = [
          docgen
        ];
        shellHook = ''
          # Installs a docgen pre-commit hook
          ${git-hooks-check.shellHook}
        '';
      };
    };
    checks.${system} = {
      inherit git-hooks-check;
    };
  };
}
```

### Credits

- [lemmy-help](https://github.com/numToStr/lemmy-help)
- TJ's [docgen](https://github.com/tjdevries/tree-sitter-lua#docgen) module
- [mini.doc](https://github.com/echasnovski/mini.nvim#minidoc) from `mini.nvim` plugin


### License

This project is [licensed](./LICENSE) according to GPL version 2
or (at your option) any later version.

lemmy-help (from which this project is forked)
is [licensed](./lemmy-help-LICENSE) according to MIT.
