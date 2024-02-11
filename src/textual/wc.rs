use std::fs;

fn sanitize_flags<'a>(flags: &'a str) -> Vec<&'a str> {
    let mut clean_flags = flags.split("").collect::<Vec<&str>>();
    clean_flags.retain(|&x| x != "" && x!= "-");
    return clean_flags;
}

pub fn words(file_paths: &[String]) {
    let mut flags: Vec<&str> = Vec::new();
    if file_paths[0].starts_with("-") {
        flags = sanitize_flags(&file_paths[0]);
    }

    for (i, file_path) in file_paths.iter().enumerate() {

        if flags.len() != 0 && i == 0 {
            continue;
        }

        let contents = match fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("Oopise daisy! Could not read file {}", err);
            }
        };
        
        let mut lc = 0;
        let mut wc = 0;
        let bc = contents.as_bytes().len();
        contents.split("\n").for_each(|line| {
            lc += 1;
            wc += line.split(" ").collect::<Vec<&str>>().len();
        });

        let l = flags.contains(&"l");
        let w = flags.contains(&"w");
        let b = flags.contains(&"b");

        if flags.len() == 0 || (l && w && b) {
            println!("l{} w{} b{} {}", lc, wc, bc, file_path);
        } 
        else if l {
            if w && !b {
                println!("l{}, w{} {}", lc, wc, file_path);
            }
            else if b && !w {
                println!("l{}, b{} {}", lc, bc, file_path);
            }
            else {
                println!("l{} {}", lc, file_path);
            }
        } else if w {
            if b && !l {
                println!("w{}, b{} {}", wc, bc, file_path);
            } else {
                println!("w{} {}", wc, file_path);
            }
        } else {
            println!("b{} {}", bc, file_path);
        }
    };
}
