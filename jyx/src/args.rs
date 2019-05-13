use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jyx")]
pub struct Opt {
    /// Input files to parse
    #[structopt(short = "i", long = "input", help = "Input files")]
    pub inputs: Vec<PathBuf>,

    /// Input files to parse
    #[structopt(
        short = "o",
        long = "output",
        help = "Output file to write if not stdout"
    )]
    pub output: Option<PathBuf>,
}

impl Opt {
    pub fn new() -> Self {
        Opt::from_args()
    }
}
