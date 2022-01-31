use clap::Parser;

//TODO Check if the visibility configured is idiomatic

/// Load generator for MongoDB Atlas built using Rust
#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    /// MongoDB connection string
    #[clap(
        short,
        long,
        default_value = "mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false"
    )]
    pub(crate) conn: String,

    /// Duration in seconds. Default 120
    #[clap(short, long, parse(try_from_str), default_value_t = 120)]
    pub(crate) duration: usize,

    /// Number of top level fields in the document. Default 10
    #[clap(short, long, parse(try_from_str), default_value_t = 10)]
    pub(crate) num_fields: u16,

    /// Nesting depth. Default 0
    #[clap(long, parse(try_from_str), default_value_t = 0)]
    pub(crate) nest_depth: u8,

    /// Ratio of Inserts. Default 100
    #[clap(short, long, parse(try_from_str), default_value_t = 100)]
    pub(crate) inserts: usize,

    /// Ratio of Updates. Default 0
    #[clap(short, long, parse(try_from_str), default_value_t = 0)]
    pub(crate) updates: usize,

    /// Ratio of Queries. Default 0
    #[clap(short, long, parse(try_from_str), default_value_t = 0)]
    pub(crate) queries: usize,

    /// Length of text fields in bytes. Default 30
    #[clap(long, parse(try_from_str), default_value_t = 30)]
    pub(crate) text_size: usize,

    /// Namespace to use. Default rmdb.load
    #[clap(long, default_value = "rmdb.load")]
    pub(crate) namespace: String,

    /// Print sample document
    #[clap(short, long)]
    pub(crate) print: Option<bool>,

    /// Number of threads. Default 3
    #[clap(short, long, parse(try_from_str), default_value_t = 3)]
    pub(crate) threads: usize,

    /// Run ID start. Default is 0
    #[clap(short, long, parse(try_from_str), default_value_t = 1000)]
    pub(crate) run_id_start: usize,

    /// Add BLOB filed
    #[clap(short, long)]
    pub(crate) binary: Option<bool>,
}
