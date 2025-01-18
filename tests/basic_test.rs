use tprint::{TPrint, TPrintAlign, TPrintOutputString};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn str_output() {
    let str_output = Rc::new(RefCell::new(TPrintOutputString::new()));
    let mut str_tprint = TPrint::new_with_output(str_output.clone(), true, true, 0, 1);

    str_tprint.column_add("Left", TPrintAlign::Left, TPrintAlign::Left);
    str_tprint.column_add("Center", TPrintAlign::Center, TPrintAlign::Center);
    str_tprint.column_add("Right", TPrintAlign::Right, TPrintAlign::Right);
    str_tprint.add_data("Test1");
    str_tprint.add_data("Test2");
    str_tprint.add_data("Test3");

    str_tprint.print();

    assert_eq!(str_output.borrow().get_str(), "+=======+========+=======+\n| Left  | Center | Right |\n+=======+========+=======+\n| Test1 | Test2  | Test3 |\n+-------+--------+-------+\n");
}