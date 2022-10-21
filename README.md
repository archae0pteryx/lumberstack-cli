# Lumberstack
# !!!README is WIP!!!

### Quickstart

```shell
lumberstack 0.2.3
Generator for Opinionated RedwoodJS Projects

USAGE:
    lumberstack [OPTIONS] [--] [NAME]

ARGS:
    <NAME>    Project name and path

OPTIONS:
    -c, --config <CONFIG>                        Specifiy a config from file
        --clean                                  Remove previous artifacts
    -h, --help                                   Print help information
    -l, --log-file <LOG_FILE>                    Log output to file
    -q, --quiet                                  Less output per occurrence
    -s, --skip-tags <SKIP_TAGS>...               Tasks to skip
        --skip-checks                            Skip system checks
    -t, --tags <TAGS>...                         Specific tags to run
        --template-version <TEMPLATE_VERSION>    Specify a template version
    -v, --verbose                                More output per occurrence
    -V, --version                                Print version information
```

### Development (where is what)

- `src/app_config` Main app config (flags, constants, etc). [more info](#)
- `src/system` All things system (file, shell, deps, ect). [more info](#)
- `src/task_definitons` Where all tasks live. [more info](#)
  - `ansible/` Core task engine. [more info](#)
  - `templates/` Template processing (var interpolation) [more_info](#)
  - `redwood/` All things redwood related. [more info](#)
  - `prisma/` All things db related. [more info](#)

TODO:
- 'splain these
