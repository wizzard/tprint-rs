use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};

/// Defines the methods that a TPrintOutput object must implement.
pub trait TPrintOutput {
    fn print_str(&mut self, s: &str) -> Result<(), io::Error>;
}

/// An struct that prints output to stdout (default).
pub struct TPrintOutputStdout {}

impl TPrintOutputStdout {}

impl TPrintOutput for TPrintOutputStdout {
    fn print_str(&mut self, s: &str) -> Result<(), io::Error> {
        print!("{}", s);
        Ok(())
    }
}

/// A struct that stores the output in a string.
pub struct TPrintOutputString {
    str: String,
}

impl TPrintOutputString {
    /// Creates a new TPrintOutputString object.
    pub fn new() -> Self {
        TPrintOutputString {
            str: String::new(),
        }
    }

    /// Returns the stored string
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
    fn print_str(&mut self, s: &str) -> Result<(), io::Error> {
        self.str.push_str(s);
        Ok(())
    }
}

/// A struct that writes the output to a file.
pub struct TPrintOutputFile {
    f: File,
}

impl TPrintOutputFile {
    /// Creates a new TPrintOutputFile object.
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
    fn print_str(&mut self, s: &str) -> io::Result<()> {
        self.f.write_all(s.as_bytes())
    }
}