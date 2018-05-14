extern crate clap;

use clap::{Arg, App};
use std::env::{self, VarError};

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
  let client_id_url = append_client_id(url.to_string()).expect("Must have SOUNDCLOUD_CLIENT_ID as an env variable!");
  println!("{}", client_id_url); 
}

// todo: make the result have a better error than VarError
fn append_client_id(mut url: String) -> Result<String, VarError> {
  // maybe store this env in another way
  let client_id = env::var("SOUNDCLOUD_CLIENT_ID")?;
  url.push_str("?client_id=");
  url.push_str(&client_id);
  return Ok(url);
}