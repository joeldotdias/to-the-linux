pub mod config;
pub mod files_stuff;

use std::io::{ self, BufRead };
use config::{ Config, Command };
use files_stuff::{
    cat::{ cat_read, cat_write, cat_append },
    wc::words,
    head_tail::{ head, tail }
};

pub fn take_to_op(config: &Config) {
    
    match config.command {
        Command::Cat => {
            if config.opts.len() == 1 {
                let file_path = get_full_path(&config.opts[0]);
                cat_read(&file_path);
            }
            else if config.opts.len() == 2 {
                let file_path = get_full_path(&config.opts[1]);
                let mut to_write = get_input();
                
                match config.opts[0].as_str() {
                    "to" => cat_write(&to_write, &file_path),
                    "on" => cat_append(&mut to_write, &file_path),
                    _ => {
                        println!("Incorrect args provided");
                        std::process::exit(1);
                    }
                }
            }  
        }

        Command::Wc => {
            let file_paths = config.opts.iter().map(|f| {
                get_full_path(f)
            }).collect::<Vec<String>>();
            
            words(&file_paths);
        }

        Command::Head => {
            let file_path = config.opts.iter().map(|f| {
                get_full_path(f)
            }).collect::<Vec<String>>();

            head(&file_path);
        }

        | Command::Tail => {
            let file_path = config.opts.iter().map(|f| {
                get_full_path(f)
            }).collect::<Vec<String>>();
            tail(&file_path);
        }

        Command::Help => {

        }
    }

}

fn get_full_path(file_path: &str) -> String {
    let mut full_path = String::from(file_path);

    if !file_path.starts_with("-") && !file_path.contains(".") {
        full_path.push_str(".txt");
    }
    return full_path;
}

fn get_input() -> Vec<String> {
    let mut text_buf: Vec<String> = Vec::new();
    
    loop {
        let curr_line = io::stdin()
            .lock().lines().next()
            .unwrap().unwrap();
        
        if curr_line.as_str() == ":q" {
            break;
        }
        text_buf.push(curr_line);
    }

    return text_buf;
}
