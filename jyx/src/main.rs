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
    let mut result: Document = Document::default();

    for (i, path) in opt.inputs.iter().enumerate() {
        let mut doc: Document = format::Format::parse_file(path)?;
        if i < opt.filters.len() {
            doc = doc.select(&opt.filters[i])?.clone();
        }
        result = result.merge(doc);
    }

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
