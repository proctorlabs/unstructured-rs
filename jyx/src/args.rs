use crate::format::Format;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jyx", rename_all = "kebab_case")]
pub struct Opt {
    /// Input files to parse
    #[structopt(short = "i", long = "input", help = "Input files")]
    pub inputs: Vec<PathBuf>,

    /// Filter to apply to input files
    #[structopt(short = "f", long = "filter", help = "Input filter")]
    pub filter: Option<String>,

    /// The format of stdin input.
    #[structopt(
        raw(possible_values = "&Format::variants()", case_insensitive = "true"),
        short = "s",
        long = "stdin-format"
    )]
    pub stdin_format: Option<Format>,

    /// The format of the generated output.
    #[structopt(
        raw(possible_values = "&Format::variants()", case_insensitive = "true"),
        long = "format",
        default_value = "PrettyJson"
    )]
    pub output_format: Format,

    /// Output file, uses STDOUT if not specified
    #[structopt(
        short = "o",
        long = "output",
        help = "Output file to write if not stdout"
    )]
    pub output_file: Option<PathBuf>,
}

impl Opt {
    pub fn new() -> Self {
        Opt::from_args()
    }
}
