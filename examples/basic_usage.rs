use tprint::{TPrint, TPrintAlign, TPrintOutputString};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {

    TPrint::new(true, true, 20, 3).
        column_add("Name", TPrintAlign::Right, TPrintAlign::Left).
        column_add("Age", TPrintAlign::Center, TPrintAlign::Center).
        column_add("City", TPrintAlign::Left, TPrintAlign::Right).
        column_add("Job", TPrintAlign::Center, TPrintAlign::Left).
        column_add("ℕ𝕠𝕥𝕖𝕤", TPrintAlign::Left, TPrintAlign::Center).
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


    let str_output = Rc::new(RefCell::new(TPrintOutputString::new()));
    let mut str_tprint = TPrint::new_with_output(str_output.clone(), true, true, 0, 1);

    str_tprint.column_add("Left", TPrintAlign::Left, TPrintAlign::Left);
    str_tprint.column_add("Center", TPrintAlign::Center, TPrintAlign::Center);
    str_tprint.column_add("Right", TPrintAlign::Right, TPrintAlign::Right);
    str_tprint.add_data("");
    str_tprint.add_data("");
    str_tprint.add_data("");

    let j:i64 = 10;
    const MAX:u32 = 10;
    for i in 0..MAX {
        str_tprint.add_data(j.pow(i).to_string());
        str_tprint.add_data(j.pow(i).to_string());
        str_tprint.add_data(j.pow(MAX-i-1).to_string());
    }
    str_tprint.print();

    println!("Printing string:\n{}", str_output.borrow().get_str());
}