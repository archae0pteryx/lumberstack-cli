# Lumberstack CLI

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

Development:

- Node v14 & Yarn
- Install Rust
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

```shell
# OSX enable running non-codesigned bins (warning!)
spctl developer-mode enable-terminal
# security & privacy > privacy > developer tools > iterm > "allow app to run..."

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
