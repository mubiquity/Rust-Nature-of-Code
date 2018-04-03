extern crate nature_of_code;

#[macro_use]
extern crate clap;

use nature_of_code::random_walker;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    if matches.is_present("walker") {
        random_walker::run(1000);
    }
}
