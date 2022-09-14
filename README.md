# Lumberstack CLI

Generate a full featured, fully customizable redwoodjs app in ~5min!

---

## Requirements

- Node 14
- Yarn
- Docker

## Quick Usage

- Download the latest release for your system
- Execute the binary `./lumberstack <app-name>`

Important note:
Lumberstack is not codesigned by apple. The following instructions should get to around their forces.

```shell
# OSX enable running non-codesigned bins (warning!)
# Run in terminal to disable for session
spctl developer-mode enable-terminal
# Then goto
# security & privacy > privacy > developer tools > iterm > "allow app to run..."
# Run again.
```

## Basic Usage

- The Template files are kept separate and can be versioned. See [template repo](https://github.com/codingzeal/lumberstack-templates) for more deets.
- Lumberstack uses a configuration manifest (json) that is completely customizable.
- Generate default manifest with `./lumberstack --init`

Manifest Syntax

```json
# Top level key - value pairs are interpolated with variables in template files.
{
    app_name: string,
    my_var2: string, # all vars coerced into strings at this time
    builder: [BuildStep]
    ...
}

# TODO: Document all types (see default manifest for examples)

```

Avail options run help

```shell
# lumberstack --help
lumberstack 0.1.0-beta.1
Opinionated typescript project generator with a RedwoodJS core

USAGE:
    lumberstack [OPTIONS] [NAME]

ARGS:
    <NAME>    Project name and path. Overrides manifest value (if present)

OPTIONS:
    -c, --config <CONFIG>    Load config from file
    -d, --disable-checks     Disable system checks
    -h, --help               Print help information
    -o, --only <ONLY>        Run tag(s) (comma separated)
    -q, --quiet              Less output per occurrence
    -v, --verbose            More output per occurrence
    -V, --version            Print version information
```

## Development with Rust

- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

```shell
# build & run
cargo run -- myapp # name 'myapp' is ignored in git

# with args (see: --help)
cargo run -- myapp -v --clean

# Releasing:

# Update cargo.toml

# Build
cargo build --release

# Tag
git tag <semver>

# Push tag
git push origin --tag <semver>

# Release in Github

# OSX Code signing
codesign -s "<developer_id>" target/release/lumberstack

# Homebrew
brew tap archae0pteryx/lumberstack-cli
brew install lumberstack-cli

# cleanup brew
brew uninstall --force lumberstack-cli
brew untap archae0pteryx/lumberstack-cli
```
