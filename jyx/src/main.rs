mod args;

use args::Opt;
use unstructured::Document;

fn main() {
    let opt = Opt::new();
    let mut docs: Vec<Document> = vec![];
    for path in opt.inputs.iter() {
        let doc = serde_any::from_file(path).unwrap();
        docs.push(doc);
    }
    println!("{:?}", docs);
}
