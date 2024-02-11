pub mod config;
pub mod textual;

use config::Config;
use textual::{
    cat::{ cat_read, cat_write, cat_append },
    wc::words
};

pub fn take_to_op(config: &Config) {
    
    match config.command.as_str() {
        "cat" => {
            if config.opts.len() == 1 {
                let file_path = get_full_path(&config.opts[0]);
                cat_read(&file_path);
            }
            else if config.opts.len() == 2 {
                let file_path = get_full_path(&config.opts[1]);
                match config.opts[0].as_str() {
                    "to" => cat_write(&file_path),
                    "on" => cat_append(&file_path),
                    _ => {
                        println!("Incorrect args provided");
                        std::process::exit(1);
                    }
                }
            }  
        },
        "wc" => {
            let file_paths = config.opts.iter().map(|f| {
                get_full_path(f)
            }).collect::<Vec<String>>();
            
            words(&file_paths);
        }
        _ => {}
    }

}

fn get_full_path(file_path: &str) -> String {
    let mut full_path = String::from(file_path);

    if !file_path.starts_with("-") && !file_path.contains(".") {
        full_path.push_str(".txt");
    }
    return full_path;
}