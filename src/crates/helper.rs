use std::fs::File;
use std::io::{Error, Write};

pub trait HelperInterface {
    fn open(&self, path: &str) -> Result<File, String>;
    fn read(&self, line: Result<String, Error>) -> Result<String, String>;
    fn open_or_create(&self, path: &str) -> Result<File, String>;
    fn write(&self, file: &mut File, line: String) -> Result<(), String>;
}

pub struct Helper {}

impl HelperInterface for Helper {
    fn open(&self, path: &str) -> Result<File, String> {
        match File::open(path) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => return Ok(file),
        }
    }

    fn read(&self, line: Result<String, Error>) -> Result<String, String> {
        match line {
            Err(e) => return Err(e.to_string()),
            Ok(data) => return Ok(data),
        }
    }

    fn open_or_create(&self, path: &str) -> Result<File, String> {
        match File::create(path) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => return Ok(file),
        }
    }

    fn write(&self, file: &mut File, line: String) -> Result<(), String> {
        match file.write_fmt(format_args!("{}\n", line)) {
            Err(e) => return Err(e.to_string()),
            Ok(()) => return Ok(()),
        }
    }
}
