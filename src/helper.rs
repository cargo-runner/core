use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::global::APP_CONFIG;

pub fn append_new_line(data: &str) {
    APP_CONFIG
        .lock()
        .unwrap()
        .push_str(&(data.to_string() + "\n"));
}

pub fn read_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (number, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                append_new_line(&text);
            }
            Err(_) => println!("Error reading line {}", number + 1),
        }
    }
    Ok(())
}
