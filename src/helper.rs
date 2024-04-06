use std::{
    error::Error,
    fs::File,
    io::BufRead,
    io::BufReader,
    sync::{Arc, Mutex},
};

pub type GlobalString = Arc<Mutex<String>>;

pub fn append_to_global_str(state: &GlobalString, data: &str) {
    // Append the line and a newline character
    state.lock().unwrap().push_str(&(data.to_string() + "\n"));
}

pub fn read_file(filename: &str, state: &GlobalString) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                // Append the content of each line to the global string
                append_to_global_str(state, &content);
            }
            Err(_) => println!("Error reading line {}", index + 1),
        }
    }
    Ok(())
}
