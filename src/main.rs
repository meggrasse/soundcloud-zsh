extern crate clap;

use clap::{Arg, App};

fn main() {
  // do i need this much data?
  let matches = App::new("soundcloud-zsh")
    .version("0.1.0")
    .author("meg grasse <meggrasse@gmail.com>")
    .about("soundcloud from the command line")
    .arg(Arg::with_name("URL")
      .required(true)
      .takes_value(true)
      .index(1)
      .help("url to stream"))
    .get_matches();
  // i think unwrapping here is fine since clap should validate arguments
  let url = matches.value_of("URL").unwrap();
  println!("{}", url);
}