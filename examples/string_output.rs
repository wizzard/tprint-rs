use tprint::{TPrint, TPrintAlign, TPrintOutputString, TPrintBordersUnicode};
use std::cell::RefCell;
use std::rc::Rc;

fn main() -> std::io::Result<()> {
    let str_output = Rc::new(RefCell::new(TPrintOutputString::new()));
    let borders = Rc::new(RefCell::new(TPrintBordersUnicode {}));
    let mut str_tprint = TPrint::new_with_borders_output(borders, str_output.clone(), true, true, 0, 1);

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
    str_tprint.print()?;

    println!("{}", str_output.borrow().get_str());

    Ok(())
}