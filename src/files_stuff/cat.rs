use std::{
    fs::{ self, OpenOptions },
    io::{ self, BufRead, Write }
};


pub fn cat_read(file_path: &str) {
    
    let contents = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Oopise daisy! Could not read file {}", err);
        }
    };

    println!("{}", contents);
}

pub fn cat_write(file_path: &str) {
    let mut text_buf: Vec<String> = Vec::new();

    loop {
        let curr_line = io::stdin()
            .lock().lines().next()
            .unwrap().unwrap();
        
        if curr_line.as_str() == ":q" {
            fs::write(file_path, text_buf.join("\n")).unwrap();
            std::process::exit(0);
        }
        text_buf.push(curr_line);
    }
}

pub fn cat_append(file_path: &str) {
    let mut text_buf: Vec<String> = Vec::new();

    let mut i = 0;
    loop {
        let mut curr_line = io::stdin()
            .lock().lines().next()
            .unwrap().unwrap();
        
        if curr_line.as_str() == ":q" {
            let mut file = OpenOptions::new()
                .append(true)
                .open(file_path )
                .unwrap();
            
            file.write(text_buf.join("\n").as_bytes()).unwrap();
            std::process::exit(0);
        }

        if i == 0 {
            curr_line.insert_str(0, "\n");       
        }
        text_buf.push(curr_line);
        i += 1;
    }
}
