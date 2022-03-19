use std::io::{ErrorKind, Error};
use std::process::exit;
use std::path::PathBuf;

pub fn err_exit(msg: &str) {
    let err = Error::new(ErrorKind::Other, msg);
    println!("Error: {}", err); 
    exit(1);
}

pub fn path_buf_to_string(path: &PathBuf) -> String {
    path.as_path().display().to_string()
}