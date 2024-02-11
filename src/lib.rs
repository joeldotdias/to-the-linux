pub mod config;
pub mod textual;

use config::Config;
use textual::cat::{ cat_read, cat_write, cat_append };

pub fn take_to_op(config: &Config) {
    
    match config.command.as_str() {
        "cat" => {
            if config.opts.len() == 1 {
                cat_read(&config.opts[0]);
            }
            else if config.opts.len() == 2 {
                match config.opts[0].as_str() {
                    "to" => cat_write(&config.opts[1]),
                    "on" => cat_append(&config.opts[1]),
                    _ => {
                        println!("Incorrect args provided");
                        std::process::exit(1);
                    }
                }
            }  
        }
        _ => {}
    }

}