use std::error::Error;

use rx::{config::Config, global::APP_CONFIG, helper::read_file};

fn main() -> Result<(), Box<dyn Error>> {
    // Assume you have this filename from somewhere
    let filename = "rx.toml";
    read_file(filename)?;

    // Lock the Mutex and get a reference to the String, don't try to move it
    let file_content = APP_CONFIG.lock().unwrap();

    let config: Config = toml::from_str(&file_content)?;

    //println!("{:#?}", config.commands.run.configs.get("leptos").unwrap());
    //println!("{:#?}", config.commands.run.configs.get("dioxus").unwrap());
    //println!("{:#?}", config.commands.run.configs.get("default").unwrap());

    let config2: Config = Config::default();
    println!(
        "{:#?}",
        config2.commands.run.configs.get("default").unwrap()
    );
    println!(
        "{:#?}",
        config2.commands.test.configs.get("default").unwrap()
    );
    println!(
        "{:#?}",
        config2.commands.build.configs.get("default").unwrap()
    );

    Ok(())
}
