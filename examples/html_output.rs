use tprint::{TPrint, TPrintAlign, TPrintBordersHTML, TPrintOutputFile};
use std::cell::RefCell;
use std::rc::Rc;

const OUT_FILE: &str = "/tmp/tprint.html";

fn main() {
    let output = Rc::new(RefCell::new(TPrintOutputFile::new(OUT_FILE).unwrap()));
    let borders = Rc::new(RefCell::new(TPrintBordersHTML {}));
    TPrint::new_with_borders_output(borders, output, true, true, 0, 1).
        column_add("Name", TPrintAlign::Center, TPrintAlign::Left).
        column_add("Age", TPrintAlign::Center, TPrintAlign::Center).
        column_add("City", TPrintAlign::Center, TPrintAlign::Right).
        column_add("Job title", TPrintAlign::Center, TPrintAlign::Left).
        column_add("ℕ𝕠𝕥𝕖𝕤", TPrintAlign::Center, TPrintAlign::Center).
        add_data("Fernando Alameda").
        add_data(42).
        add_data("London").
        add_data("Software developer").
        add_data("Ⓗⓔⓛⓛⓞ Ⓦⓞⓡⓛⓓ").
        add_data("Teo Ocaña").
        add_data(1).
        add_data("Madrid").
        add_data("Accountant").
        add_data("❤️😴🤦🏼‍♂️").
        print();
    println!();
}