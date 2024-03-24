use std::{
    io::Write,
    fs::{ self, OpenOptions }
};

pub fn cat_read(file_paths: &[String]) -> String{
    let mut contents = String::new();

    for (i, file_path) in file_paths.iter().enumerate() {
        match fs::read_to_string(file_path) {
            Ok(file_contents) => {
                contents.push_str(&file_contents);
            }
            Err(err) => {
                panic!("Oopise daisy! Could not read file {}", err);
            }
        };
        
        if i < (file_paths.len() - 1) {
            contents.push('\n');
        }
    }
    
    contents
}

pub fn cat_write(text_buf: &[String], file_path: &str) {
    if let Err(err) = fs::write(file_path, text_buf.join("\n")) {
        panic!("{}", err)
    };
}

pub fn cat_append(text_buf: &mut [String], file_path: &str) {
    text_buf[0].insert(0, '\n');

    let mut req_file = match OpenOptions::new()
        .append(true)
        .open(file_path ) {
            Ok(file) => file,
            Err(err) => {
                panic!("{}", err);
            }
        };

    if let Err(err) = req_file.write(text_buf.join("\n").as_bytes()) {
        panic!("Failed to write to {}: {}", file_path, err)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_cat_read_work() {
        let file_paths = vec!["cat_test.txt".to_string(), "cat_test2.txt".to_string()];
        let file_contents = String::from("Hello,\r\nmay this\r\nwork.\nHello everybody\r\nHope this works");
        assert_eq!(cat_read(&file_paths), file_contents);
    }

    #[test]
    fn does_cat_write_work() {
        let text_buf = vec!["new file".to_string(), "go brrrr".to_string()];
        let file_path = "tiny_file.txt";
        cat_write(&text_buf, file_path);
        let file_contents = fs::read_to_string(file_path).unwrap();
        assert_eq!(text_buf.join("\n"), file_contents);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn does_cat_append_work() {
        let mut contents = vec!["prev stuff".to_string(), "pre append".to_string(), "post append".to_string(), "new stuff".to_string()];
        let file_path = "tinier_file.txt";
        cat_write(&contents[0..2], file_path);
        cat_append(&mut contents[2..], file_path);
        let file_contents = fs::read_to_string(file_path).unwrap();
        assert_eq!(contents.join("\n").replace("\n\n", "\n"), file_contents);
        fs::remove_file(file_path).unwrap();
    }
}
