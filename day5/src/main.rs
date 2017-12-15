use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn run(program: &mut Vec<i32>) -> i32 {
    let mut ip: i32 = 0;
    let mut count: i32 = 0;

    loop {
        if ip < 0 || ip >= program.len() as i32 {
            return count;
        }

        let next_ip = ip + program[ip as usize];
        program[ip as usize] += 1;
        ip = next_ip;
        count += 1;
    }
}

fn handle_file(filename: &String) {
    let f = File::open(filename).expect("file not found.");
    let reader = BufReader::new(f);

    let mut program: Vec<i32> = reader.lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect();

    println!("{}: {} jumps.", filename, run(&mut program));
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
        assert_eq!(run(&mut vec![0, 3, 0, 1, -3]), 5);
    }
}
