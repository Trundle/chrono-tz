use std::env::args_os;
use std::io::{Write, stderr};
use std::process::exit;

extern crate datetime;
extern crate getopts;
extern crate phf_codegen;
extern crate zoneinfo_parse;

#[macro_use]
extern crate quick_error;

mod data_crate;
use data_crate::DataCrate;

mod errors;
use errors::Error;

#[macro_use]
mod util;


fn main() {
    if let Err(e) = build_data_crate() {
        println_stderr!("{}", e);
        exit(1);
    }
}

fn build_data_crate() -> Result<(), Error> {
    let mut opts = getopts::Options::new();
    opts.reqopt("o", "output", "directory to write the crate into", "DIR");

    let matches = opts.parse(args_os().skip(1))?;
    let data_crate = DataCrate::new(matches.opt_str("output").unwrap(), &matches.free)?;
    data_crate.run()?;

    println!("All done.");
    Ok(())
}
