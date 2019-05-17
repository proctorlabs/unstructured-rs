/*!
jyx is a CLI tool for manipulating data of various formats.
*/

mod args;
mod format;

#[macro_use]
extern crate clap;

use args::Opt;
use std::fs::File;
use std::io::prelude::*;
use unstructured::Document;

fn main() -> Result<(), String> {
    let opt = Opt::new();
    let mut docs: Vec<Document> = vec![];

    if let Some(in_format) = opt.stdin_format {
        docs.push(in_format.parse_stdin()?);
    }

    for path in opt.inputs.iter() {
        docs.push(format::Format::parse_file(path)?);
    }

    let filter = match opt.filter {
        Some(s) => s,
        None => "*".to_string(),
    };

    let result = Document::filter(&docs, &filter)?;

    let output = opt.output_format.serialize(&result)?;

    match opt.output_file {
        Some(s) => {
            let mut file = File::create(s).map_err(|e| format!("{}", e))?;
            file.write_all(output.as_bytes())
                .map_err(|e| format!("{}", e))?;
        }
        None => println!("{}", output),
    };

    Ok(())
}
