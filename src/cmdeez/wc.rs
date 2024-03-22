use core::panic;
use std::{ fs, vec };

fn make_stats<'a, I> (flags: Option<&'a str>, file_contents: I) -> impl Iterator<Item = String> + '_
    where
        I: Iterator<Item = String> + 'a
    {
        let default_stats = vec!["l", "w", "b"];

        let reqd_stats = match flags {
            Some(flags) => {
                flags.split("").filter_map(|ch| {
                    (!ch.is_empty() && ch!= "-").then_some(ch)
                }).collect()
            }
            None => default_stats
        };

        let stats = file_contents.map(move |contents| {
            get_stat_str(&reqd_stats[0..], &contents)
        });

        stats
}

pub fn word_stats(file_paths: &[String], piped_input: Option<String>) -> String {
    let flags: Option<&str> = if file_paths[0].starts_with('-') {
        Some(&(file_paths[0]))
    } else {
        None
    };

    let files_start_idx = match flags {
        Some(_) => 1,
        None => 0
    };

    match piped_input {
        Some(pipe) => {
            let ret_str = make_stats(flags, vec![pipe].iter().map(|f| f.into())).collect::<Vec<String>>();
            ret_str.join("")
        },
        None => {
            let contents = file_paths[files_start_idx..].iter().map(|path| {
                match fs::read_to_string(path) {
                    Ok(file_contents) => file_contents,
                    Err(err) => panic!("{}", err)
                }
            });

            let mut stats = make_stats(flags, contents).collect::<Vec<String>>();

            for i in 0..stats.len() {
                stats[i].push_str(&(file_paths[i + files_start_idx]));
            }

            stats.join("\n")
        }
    }
}

fn get_stat_str(flags: &[&str], file_contents: &str) -> String {
    let mut stat_str = String::new();

    let file_lines = file_contents.lines();
    let (mut lc, mut wc, bc) = (0, 0, file_contents.as_bytes().len());
    file_lines.for_each(|line| {
        lc += 1;
        wc += line.split(' ').count();
    });

    flags.iter().for_each(|flag| {
        match *flag {
            "l" => stat_str.push_str(format!("l{} ", lc).as_str()),
            "w" => stat_str.push_str(format!("w{} ", wc).as_str()),
            "b" => stat_str.push_str(format!("b{} ", bc).as_str()),
            _ => panic!("{} doesn't exist", flag)
        }
    });

    stat_str
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_wc() {
        let expected_result = "l4 w13 b74 wc_test.txt".to_owned();
        assert_eq!(word_stats(&(vec!["wc_test.txt".into()]), None), expected_result);
    }

    #[test]
    fn wc_with_flags() {
        let expected_result = "l4 b74 wc_test.txt".to_owned();
        assert_eq!(word_stats(&(vec!["-lb".into(), "wc_test.txt".into()]), None), expected_result);
    }
}
