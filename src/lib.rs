//! tprint is a simple Rust crate to print tabular data.

mod tprint;
mod column;
mod output;
mod borders;
mod utils;

pub use tprint::TPrint;
pub use column::TPrintAlign;

pub use output::{TPrintOutputString, TPrintOutputFile};

pub use borders::{TPrintBordersUnicode, TPrintBordersHTML};