use std::{
    fs::{ self, OpenOptions },
    io::{ self, BufRead, Write }
};

fn get_full_path(file_path: &str) -> String {
    let mut full_path = String::from(file_path);

    if !file_path.contains(".") {
        full_path.push_str(".txt");
    }
    return full_path;
}

pub fn cat_read(file_path: &str) {
    let full_path = get_full_path(file_path);

    let contents = match fs::read_to_string(full_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Oopise daisy! Could not read file {}", err);
        }
    };

    println!("{}", contents);
}

pub fn cat_write(file_path: &str) {
    let full_path = get_full_path(file_path);
    let mut text_buf: Vec<String> = Vec::new();

    loop {
        let curr_line = io::stdin()
            .lock().lines().next()
            .unwrap().unwrap();
        
        if curr_line.as_str() == ":q" {
            fs::write(full_path, text_buf.join("\n")).unwrap();
            std::process::exit(0);
        }
        text_buf.push(curr_line);
    }
}

pub fn cat_append(file_path: &str) {
    let full_path = get_full_path(file_path);
    let mut text_buf: Vec<String> = Vec::new();

    let mut i = 0;
    loop {
        let mut curr_line = io::stdin()
            .lock().lines().next()
            .unwrap().unwrap();
        
        if curr_line.as_str() == ":q" {
            let mut file = OpenOptions::new()
                .append(true)
                .open(full_path)
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
