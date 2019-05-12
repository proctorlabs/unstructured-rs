#[macro_use]
extern crate clap;

mod args;

fn main() {
    args::parse_args();
}
