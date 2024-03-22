use std::io::{ self, BufRead };
use crate::{
    config::{ Config, Command },
    cmdeez::{
        cat::{ cat_read, cat_write, cat_append },
        wc::word_stats,
        head_tail::{ head, tail }
    }
};


pub fn exec_ops(configs: &[Config]) {
    let mut prev_result = String::new();
    configs.iter().for_each(|config| {
        prev_result = to_the_op(config, &prev_result)
    });

    if !prev_result.is_empty() {
        println!("{}", prev_result);
    }
}

fn to_the_op(config: &Config, prev_out: &str) -> String {
    let mut curr_out = String::new();

    match config.command {
        Command::Cat => {
            if config.opts.contains(&("to".into())) || config.opts.contains(&("on".into())) {
                let file_path = get_full_path(&config.opts[1]);
                let mut to_write = if prev_out.is_empty() { 
                    get_input()
                } else {
                    prev_out.split('\n').map(|line| String::from(line))
                        .collect::<Vec<String>>()
                };
                
                match config.opts[0].as_str() {
                    "to" => cat_write(&to_write, &file_path),
                    "on" => cat_append(&mut to_write, &file_path),
                    _ => {
                        panic!("Incorrect args provided");
                    }
                }
            } else {
                let file_paths = config.opts[0..].iter().map(|file| {
                    get_full_path(file)
                }).collect::<Vec<String>>(); 
                
                curr_out = cat_read(&file_paths);
                   
            }  
        }

        Command::Wc => {
            let file_paths = config.opts.iter().map(|f| {
                get_full_path(f)
            })
            .collect::<Vec<String>>();
            
            let piped_input = if !prev_out.is_empty() {
                Some(prev_out.into())
            } else {
                None
            };
            curr_out = word_stats(&file_paths, piped_input);
        }

        Command::Head => {
            let file_path = config.opts.iter().map(|f| {
                get_full_path(f)
            }).collect::<Vec<String>>();

            head(&file_path);
        }

        Command::Tail => {
            let file_path = config.opts.iter().map(|f| {
                get_full_path(f)
            }).collect::<Vec<String>>();
            tail(&file_path);
        }

        Command::Help => {

        }
    }

    curr_out
}

fn get_full_path(file_path: &str) -> String {
    let mut full_path = String::from(file_path);

    if !file_path.starts_with('-') && !file_path.contains('.') {
        full_path.push_str(".txt");
    }
    
    full_path.replace(".\\", "")
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

    text_buf
}
