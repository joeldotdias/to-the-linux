use std::{
    io::Write,
    fs::{ self, OpenOptions }
};


pub fn cat_read(file_path: &str) -> String{
    
    let contents = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Oopise daisy! Could not read file {}", err);
        }
    };

    return contents;
}

pub fn cat_write(text_buf: &[String], file_path: &str) {
    fs::write(file_path, text_buf.join("\n")).unwrap();
}

pub fn cat_append(text_buf: &mut [String], file_path: &str) {
    text_buf[0].insert_str(0, "\n");

    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path )
        .unwrap();
    
    file.write(text_buf.join("\n").as_bytes()).unwrap();
}
