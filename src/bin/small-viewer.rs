extern crate small_viewer;
extern crate getopts;

use std::env;
use getopts::Options;
use std::io::prelude::*;
use std::fs::File;

use small_viewer::config_reader;
use small_viewer::daemon;



extern crate time;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [-d] [-s] [COMMAND WITH PARAMETERS]", program);
    print!("{}", opts.usage(&brief));
}


fn main() {

  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("c", "config", "Configuration file path. Default to config.conf", "CONFIG");
  opts.optflag("h", "help", "print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => { panic!(f.to_string()) }
  };

  let configuration_file = matches.opt_str("c").unwrap_or(String::from("config.conf"));

  let mut configuration_string = String::new();
  match File::open(configuration_file) {
    Ok(mut f) => f.read_to_string(&mut configuration_string).unwrap(),
    Err(f) => { panic!(f.to_string()) }
  };

  let configuration = config_reader::categories(configuration_string.as_bytes());
  println!("Configuration: {:?}", configuration);

  if matches.opt_present("h") {
    print_usage(&program, opts);
    return;
  }
  daemon::startup();

}
