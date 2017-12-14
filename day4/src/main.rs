use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn check_passwd_part1(passwd: &str) -> bool {
    let mut used_words = HashSet::new();

    for word in passwd.split_whitespace() {
        match used_words.get(word) {
            Some(_) => return false,
            None => used_words.insert(word),
        };
    }

    true
}
fn handle_file(filename: &String) {
    let f = File::open(filename).expect("file not found.");
    let reader = BufReader::new(f);

    let mut valid_count = 0;
    let mut invalid_count = 0;

    for line in reader.lines() {
        let linestr = line.unwrap();

        if check_passwd_part1(&linestr) {
            valid_count += 1;
        } else {
            invalid_count += 1;
        }
    }
    println!("{}: {} valid, {} invalid", filename, valid_count, invalid_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filenames = &args.as_slice()[1.. ];
    for filename in filenames {
        handle_file(&filename);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(check_passwd_part1("aa bb cc dd ee"), true);
        assert_eq!(check_passwd_part1("aa bb cc dd aa"), false);
        assert_eq!(check_passwd_part1("aa bb cc dd aaa"), true);
    }
}
