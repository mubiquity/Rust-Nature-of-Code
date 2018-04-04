extern crate nature_of_code;

#[macro_use]
extern crate clap;

use nature_of_code::random_walker;
use nature_of_code::noise_walker;
use nature_of_code::ball;
use nature_of_code::mouse_ball;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    if matches.is_present("walker") {
        random_walker::run(5000);
    }

    if matches.is_present("noise-walker") {
        noise_walker::run(10);
    }

    if matches.is_present("ball") {
        ball::run(1000);
    }

    if matches.is_present("mouse-ball") {
        mouse_ball::run(20);
    }
}
