# MongoDB load gen: test your MongoDB cluster performance
### Written in Rust

## Usage

```shell
rust_mdb_load --help
2022-01-30 10:13:54 [rust_mdb_load] INFO  rust_mdb_load: Initializing MongoDB load generator!
rust_mdb_load 0.1.0
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
```

## Examples
```shell
cargo build
rust_mdb_load --num-fields=10 --binary=false --text-size=100 --duration=10 --namespace="mydb.mycoll"
time ./target/debug/rust_mdb_load --num-fields=10 --binary=false --text-size=100 --duration=10 --namespace="mydb.mycoll" --threads=5 --inserts=75 --queries=25
```
