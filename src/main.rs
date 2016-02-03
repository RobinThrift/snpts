extern crate getopts;
use getopts::Options;
use std::env;

mod commands;

fn print_error(txt: &str) {
    println!("ERROR: {}", txt);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        println!("Hello Help");
    }

    if !matches.free.is_empty() {
        match commands::exec_command(&matches.free[0].clone()) {
            Ok(r) => { println!("{}", r); }
            Err(e) => { print_error(e); }
        }
    } else {
        print_error("Please specify a command");
    };
}
