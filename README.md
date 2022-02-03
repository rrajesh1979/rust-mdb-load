<!-- markdownlint-configure-file {
  "MD013": {
    "code_blocks": false,
    "tables": false
  },
  "MD033": false,
  "MD041": false
} -->

<div align="center">

# MongoDB Load gen
## Test your MongoDB cluster performance
### Written in Rust 🦀

[![CI](https://github.com/rrajesh1979/rust_mdb_load/actions/workflows/ci.yml/badge.svg)](https://github.com/rrajesh1979/rust_mdb_load/actions/workflows/ci.yml) | 
[![CD](https://github.com/rrajesh1979/rust_mdb_load/actions/workflows/cd.yml/badge.svg)](https://github.com/rrajesh1979/rust_mdb_load/actions/workflows/cd.yml)

<a href="https://github.com/rrajesh1979/rust_mdb_load/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=rrajesh1979/rust_mdb_load"  alt="Contributors"/>
</a>

Made with [contrib.rocks](https://contrib.rocks).

[Key Features](#key-features) •
[Getting started](#getting-started) •
[How to use](#how-to-use) •
[Configuration](#configuration) •
[Related projects](#related-projects) •
[GitPod Environment](#gitpod-environment) •
[License](#license) •
[Code Quality](#code-quality) •

</div>

## Key Features
<div>
This is CLI tool to test your MongoDB cluster performance. This tool is built using the following set of tools
<ul>
    <li>Rust</li>
    <li>Cargo</li>
    <li>JReleaser</li>
    <li>GitHub Actions and Workflows</li>
    <li>GitPod environment</li>
</ul>
</div>


## Usage

```shell
brew update
brew install rrajesh1979/tap/rust_mdb_load
brew upgrade rrajesh1979/tap/rust_mdb_load

brew upgrade rust_mdb_load
==> Upgrading 1 outdated package:
rrajesh1979/tap/rust_mdb_load 0.0.16 -> 0.0.17
==> Downloading https://github.com/rrajesh1979/rust_mdb_load/releases/download/v0.0.17/rust_mdb_load-0.0.17-x8
==> Downloading from https://objects.githubusercontent.com/github-production-release-asset-2e65be/453402124/fb
######################################################################## 100.0%
==> Upgrading rrajesh1979/tap/rust_mdb_load
  0.0.16 -> 0.0.17

🍺  /usr/local/Cellar/rust_mdb_load/0.0.17: 5 files, 9.9MB, built in 3 seconds
==> Running `brew cleanup rust_mdb_load`...

rust_mdb_load --help
2022-01-30 10:13:54 [rust_mdb_load] INFO  rust_mdb_load: Initializing MongoDB load generator!
rust_mdb_load 0.0.13
Load generator for MongoDB Atlas using Rust

USAGE:
    rust_mdb_load [OPTIONS]

OPTIONS:
    -b, --binary <BINARY>
            Add BLOB filed

    -c, --conn <CONN>
            MongoDB connection string [default:
            mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false]

    -d, --duration <DURATION>
            Duration in seconds. Default 120 [default: 120]

    -h, --help
            Print help information

    -i, --inserts <INSERTS>
            Ratio of Inserts. Default 100 [default: 100]

    -n, --num-fields <NUM_FIELDS>
            Number of top level fields in the document. Default 10 [default: 10]

        --namespace <NAMESPACE>
            Namespace to use. Default rmdb.load [default: rmdb.load]

        --nest-depth <NEST_DEPTH>
            Nesting depth. Default 0 [default: 0]

    -p, --print <PRINT>
            Print sample document

    -q, --queries <QUERIES>
            Ratio of Queries. Default 0 [default: 0]

    -r, --run-id-start <RUN_ID_START>
            Run ID start. Default is 0 [default: 0]

    -t, --threads <THREADS>
            Number of threads. Default 3 [default: 3]

        --text-size <TEXT_SIZE>
            Length of text fields in bytes. Default 30 [default: 30]

    -u, --updates <UPDATES>
            Ratio of Updates. Default 0 [default: 0]

    -V, --version
            Print version information


rust_mdb_load --num-fields=10 --binary=false --text-size=100 --duration=10 --namespace="mybrew.mycoffee" --threads=5 --inserts=60 --queries=20 --updates=20
```

## Build yourself
```shell
# Pre-requisite: Rust tool chain
git clone https://github.com/rrajesh1979/rust_mdb_load.git
cd rust_mdb_load
cargo clean
cargo fmt --all -- --check
cargo fmt --all
cargo audit
cargo build --release
rust_mdb_load --num-fields=10 --binary=false --text-size=100 --duration=10 --namespace="mydb.mycoll"
time ./target/debug/rust_mdb_load --num-fields=10 --binary=false --text-size=100 --duration=10 --namespace="mydb.mycoll" --threads=5 --inserts=60 --queries=20 --updates=20
```
## Sample output
```shell
time rust_mdb_load --num-fields=10 --binary=false --text-size=5 --duration=20 --namespace="mybrew.mycoffee" --threads=5 --inserts=100 --queries=25 --updates=25
2022-02-02 21:18:28 [rust_mdb_load] INFO  rust_mdb_load: Initializing MongoDB load generator!
2022-02-02 21:18:28 [rust_mdb_load::mongo_load_gen] INFO  rust_mdb_load::mongo_load_gen: Clean-up of mybrew.mycoffee completed!
2022-02-02 21:18:38 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: ------------ Stats after 10 seconds -----------
2022-02-02 21:18:38 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of inserts: 38971
2022-02-02 21:18:38 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of updates: 6959
2022-02-02 21:18:38 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of queries: 9823
2022-02-02 21:18:38 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: -----------------------------------------------
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: ------------ Stats after 20 seconds -----------
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of inserts: 77426
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of updates: 15752
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of queries: 19370
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: -----------------------------------------------
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: ------------ Slow Ops during the run-----------
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of slow inserts: 14
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of slow updates: 0
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: Number of slow queries: 2
2022-02-02 21:18:48 [rust_mdb_load::stats_reporter] INFO  rust_mdb_load::stats_reporter: -----------------------------------------------
./target/debug/rust_mdb_load --num-fields=10 --binary=false --text-size=5      71.22s user 10.38s system 397% cpu 20.545 total
```


## GitPod Environment
Fork and develop online using this ready to use GitPod environment.

[![setup automated](https://img.shields.io/badge/Gitpod-ready_to_code-orange?logo=gitpod)](https://gitpod.io/from-referrer/)

## Related projects


## License

![GitHub](https://img.shields.io/github/license/rrajesh1979/rust_mdb_load)
