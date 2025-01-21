use tprint::{TPrint, TPrintAlign, TPrintOutputFile, TPrintBordersUnicode};
use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::env;
use std::io;
use std::time::UNIX_EPOCH;

const OUT_FILE: &str = "/tmp/tprint.txt";
const DEFAULT_PATH: &str = "/tmp";
const FILE_SYMBOL: &str = "\u{1F4C4}";
const DIR_SYMBOL: &str = "\u{1F4C1}";

fn main() -> io::Result<()> {
    let output = Rc::new(RefCell::new(TPrintOutputFile::new(OUT_FILE).unwrap()));
    let borders = Rc::new(RefCell::new(TPrintBordersUnicode {}));

    let mut f_tprint = TPrint::new_with_borders_output(borders, output, true, true, 0, 1);

    f_tprint.column_add("Type", TPrintAlign::Center, TPrintAlign::Center).
        column_add("Name", TPrintAlign::Center, TPrintAlign::Left).
        column_add("Size", TPrintAlign::Center, TPrintAlign::Right).
        column_add("Creation time", TPrintAlign::Center, TPrintAlign::Left);

    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { DEFAULT_PATH };

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name().into_string().unwrap();

        let duration_since_epoch = metadata.created()?.duration_since(UNIX_EPOCH).expect("Incorrect file datetime format");
        let secs = duration_since_epoch.as_secs();
        let file_date = chrono::DateTime::from_timestamp(secs as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string());

        f_tprint.add_data(if file_type.is_dir() { DIR_SYMBOL } else { FILE_SYMBOL });
        f_tprint.add_data(file_name);
        f_tprint.add_data(metadata.len());
        f_tprint.add_data(file_date);
    }

    f_tprint.print()?;

    let output = fs::read_to_string(OUT_FILE)?;
    print!("{}", output);
    println!("Content of TPrint Output file: {}", OUT_FILE);

    Ok(())
}
