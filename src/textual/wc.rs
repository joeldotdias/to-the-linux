use std::{
    collections::HashMap,
    fs
};


fn sanitize_flags<'a>(flags: &'a str) -> HashMap<&'a str, i32> {
    let mut clean_flags = HashMap::new();
    
    flags.split("").for_each(|flag| {
        if flag != "" && flag!= "-" {
            clean_flags.insert(flag, -1);
        }
    });
    return clean_flags;
}
// 34 opa
pub fn words(file_paths: &[String]) {
    let mut flags: HashMap<&str, i32> = HashMap::new();
    
    if file_paths[0].starts_with("-") {
        flags = sanitize_flags(&file_paths[0]);
    }
    let needs_l = flags.contains_key("l");
    let needs_w = flags.contains_key("w");
    let needs_b = flags.contains_key("b");


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

        if needs_l {
            flags.entry("l").and_modify(|count| *count = lc);
        }
        if needs_w {
            flags.entry("w").and_modify(|count| *count = wc as i32);
        }
        if needs_b {
            flags.entry("b").and_modify(|count| *count = bc as i32);
        }  

        
        if flags.len() == 0 {
            println!("l{} w{} b{} {}", lc, wc, bc, file_path);
            continue;
        }

        let mut base_str = String::new();
        for (flag, count) in flags.iter() {
            base_str.push_str(&format!("{}{} ", *flag, *count));
        }
        base_str.push_str(&file_path);
        println!("{}", base_str);
    };
}
