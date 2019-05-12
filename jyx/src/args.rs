use clap::App;

pub fn parse_args() {
    App::new("jyx")
        .version(crate_version!())
        .author("Phil Proctor <philliptproctor@gmail.com>")
        .about("jyx is a tool for manipulating and converting data structures.")
        .bin_name("jyx")
        .get_matches();
}
