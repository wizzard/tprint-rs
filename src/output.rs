use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};

pub trait TPrintOutput {
    fn print_str(&mut self, s: &str);
}

pub struct TPrintOutputStdout {}

impl TPrintOutputStdout {}

impl TPrintOutput for TPrintOutputStdout {
    fn print_str(&mut self, s: &str) {
        print!("{}", s);
    }
}


pub struct TPrintOutputString {
    str: String,
}

impl TPrintOutputString {
    pub fn new() -> Self {
        TPrintOutputString {
            str: String::new(),
        }
    }

    pub fn get_str(&self) -> &str {
        &self.str
    }
}

impl Default for TPrintOutputString {
    fn default() -> Self {
        Self::new()
    }
}

impl TPrintOutput for TPrintOutputString {
    fn print_str(&mut self, s: &str) {
        self.str.push_str(s);
    }
}


pub struct TPrintOutputFile {
    f: File,
}

impl TPrintOutputFile {
    pub fn new(file_name: &str) -> io::Result<Self> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)?;

        Ok(TPrintOutputFile { f })
    }
}

impl TPrintOutput for TPrintOutputFile {
    fn print_str(&mut self, s: &str) {
        self.f.write_all(s.as_bytes()).unwrap();
    }
}