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

fn check_passwd_part2(passwd: &str) -> bool {
    let mut used_words = HashSet::new();

    for word in passwd.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let normalized_word: String = chars.into_iter().collect();
        match used_words.get(&normalized_word) {
            Some(_) => return false,
            None => used_words.insert(normalized_word),
        };
    }
    true
}

fn handle_file(filename: &String) {
    let f = File::open(filename).expect("file not found.");
    let reader = BufReader::new(f);

    let mut valid_count1 = 0;
    let mut invalid_count1 = 0;

    let mut valid_count2 = 0;
    let mut invalid_count2 = 0;

    for line in reader.lines() {
        let linestr = line.unwrap();

        if check_passwd_part1(&linestr) {
            valid_count1 += 1;
        } else {
            invalid_count1 += 1;
        }

        if check_passwd_part2(&linestr) {
            valid_count2 += 1;
        } else {
            invalid_count2 += 1;
        }
    }
    println!("{} part 1: {} valid, {} invalid", filename, valid_count1, invalid_count1);
    println!("{} part 2: {} valid, {} invalid", filename, valid_count2, invalid_count2);
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

    #[test]
    fn example_part2() {
        assert_eq!(check_passwd_part2("abcde fghij"), true);
        assert_eq!(check_passwd_part2("abcde xyz ecdab"), false);
        assert_eq!(check_passwd_part2("a ab abc abd abf abj"), true);
        assert_eq!(check_passwd_part2("iiii oiii ooii oooi oooo"), true);
        assert_eq!(check_passwd_part2("oiii ioii iioi iiio"), false);
    }
}
