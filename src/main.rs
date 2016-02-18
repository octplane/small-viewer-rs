#![allow(dead_code)]

extern crate rustc_serialize;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate getopts;
extern crate urlencoded;


extern crate frank_jwt;

mod data_format;
mod daemon;
mod api;

use std::env;
use getopts::Options;

extern crate time;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [-d] [-s] [COMMAND WITH PARAMETERS]", program);
    print!("{}", opts.usage(&brief));
}


fn main() {

  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("c", "config", "Configuration file path. Default to config.ini", "CONFIG");
  opts.optflag("h", "help", "print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => { panic!(f.to_string()) }
  };

  let configuration_file = matches.opt_str("c").unwrap_or(String::from("config.ini"));

  if matches.opt_present("h") {
    print_usage(&program, opts);
    return;
  }
  daemon::startup();

}
