extern crate clap;

use std::fs;
use std::os::unix::fs::PermissionsExt;

use clap::{App, Arg};

fn validate_executable(value: String) -> Result<(), String> {
    let metadata    = try!(fs::metadata(&value).map_err(|_| format!("Invalid executable: {}", value)));
    let permissions = metadata.permissions();

    if metadata.is_file() && permissions.mode() & 0o111 != 0 {
        return Ok(());
    }

    Err(format!("Invalid executable: {}", value))
}

fn validate_port(value: String) -> Result<(), String> {
    let port = try!(value.parse::<u16>().map_err(|_| format!("Invalid port: {}", value)));

    if port > 0 {
        return Ok(());
    }

    Err(format!("Invalid port: {}", value))
}

fn validate_dir(value: String) -> Result<(), String> {
    let metadata = try!(fs::metadata(&value).map_err(|_| format!("Invalid dir: {}", value)));

    if metadata.is_dir() {
        return Ok(());
    }

    Err(format!("Invalid dir: {}", value))
}

fn main() {
    let app = App::new("Home control API")
        .about("REST API for personal home control")
        .arg(Arg::with_name("tdtool")
             .long("tdtool")
             .value_name("FILE")
             .help("Path to the tdtool executable")
             .takes_value(true)
             .required(true)
             .validator(validate_executable))
        .arg(Arg::with_name("port")
             .long("port")
             .value_name("FILE")
             .help("The port for the HTTP listener")
             .default_value("3000")
             .takes_value(true)
             .validator(validate_port))
        .arg(Arg::with_name("work-dir")
             .long("work-dir")
             .value_name("DIR")
             .help("Where to save work files")
             .default_value("/tmp")
             .validator(validate_dir));

    let matches = app.get_matches();

    let tdtool  = matches.value_of("tdtool").unwrap();
    let port    = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let workdir = matches.value_of("work-dir").unwrap();

    println!("{} {} {}", tdtool, port, workdir);
}
