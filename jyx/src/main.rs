/*!
jyx is a CLI tool for manipulating data of various formats. It is useful for converting between various formats and
filtering data ingested by any of the supported formats.

# Usage

jyx allows merging and filtering of any arbitrary number of inputs. One input source may be from stdin with an input
format specified, which, when used, will be sourced as the document at index 0 for filtering purposes.

```bash
Command line tool for manipulating data structures

USAGE:
    jyx [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <filter>                Input filter
    -i, --input <inputs>...              Input files
    -o, --output <output-file>           Output file to write if not stdout
        --format <output-format>         The format of the generated output. [default: PrettyJson]  [possible values:
                                         PrettyJson, Json, Yaml, Toml, Xml]
    -s, --stdin-format <stdin-format>    The format of stdin input. [possible values: PrettyJson, Json, Yaml, Toml, Xml]
```

## Examples

```bash
# This will print the combined result of both files to stdout
jyx -i input1.json -i input2.yaml

# Write to file instead of stdout
jyx -i input1.json -o output.json

# Convert to yaml
jyx -i input1.json -o output.yaml --format yaml

# Merge two files, write output to a yaml file, filter inputs to pull only selected fields
jyx -i Cargo.toml -i input2.json -o result.yaml --format yaml -f '[0].dependencies | [1].someData'

# Send request to XML API, convert output to JSON
curl https://raw.githubusercontent.com/danyork/sample-xml-files/master/helloworld.xml | jyx -s xml
```

# Filters

The filter syntax is inspired by jq, another great tool, but also supports [JSON Pointer syntax](https://tools.ietf.org/html/rfc6901).
```bash
# Only print the key "key" in the first input document
jyx -f [0].key

# Print the first five element in the array "array" of the first input document
jyx -f [0].array.[:5]

# Print the key "key" from the first document and elements 5-10 of the array "array" from the second document
jyx -f '[0].["key"] | [1].array.[5:10]'
# Map keys can be accessed with any of the following syntaxes: .key .["key"] /key
# Specific array indexes can be accessed with an index identifier: .key.[1]
# Alternatively, keep that index in an array: .key.[1:1]
# Array ranges are open ended on either side: .key.[:5] .key.[5:] .key.[:]
# When merging documents, later values will take precedence over earlier values. Any collision in [1].key will overwrite [0].key here: [0].key | [1].key
```
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
