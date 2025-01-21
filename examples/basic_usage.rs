use tprint::{TPrint, TPrintAlign};

fn main() {
    println!("1. Basic usage example: a table with 5 columns and 2 rows, with 10 spaces of padding from the left:");
    TPrint::new(true, true, 10, 3).
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

    println!("2. Basic usage example: a table without borders with 5 columns and 2 rows, with 10 spaces of padding from the left:");
    TPrint::new(false, true, 10, 3).
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

    println!("3. Basic usage example: a table without borders and headers with 5 columns and 2 rows, with 10 spaces of padding from the left:");
    TPrint::new(false, false, 10, 3).
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