use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn run<F>(program: &mut Vec<i32>, update: F)
    -> i32 where F : Fn(i32) -> i32 {
    let mut ip: i32 = 0;
    let mut count: i32 = 0;

    loop {
        if ip < 0 || ip >= program.len() as i32 {
            return count;
        }

        let val = program[ip as usize];
        let next_ip = ip + val;
        program[ip as usize] = update(val);
        ip = next_ip;
        count += 1;
    }
}

fn run_part1(program: &mut Vec<i32>) -> i32 {
    run(program, |o| o + 1)
}

fn run_part2(program: &mut Vec<i32>) -> i32 {
    run(program, |o| {
        if o >= 3 {
        return o - 1;
        } else {
        return o + 1;
        }
        })
}

fn handle_file(filename: &String) {
    let f = File::open(filename).expect("file not found.");
    let reader = BufReader::new(f);

    let mut p1: Vec<i32> = reader.lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut p2 = p1.to_vec();
    println!("{}: part1: {} jumps. part2 {} jumps.", filename,
             run_part1(&mut p1), run_part2(&mut p2));
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
        assert_eq!(run_part1(&mut vec![0, 3, 0, 1, -3]), 5);
    }

    #[test]
    fn example_part2() {
        assert_eq!(run_part2(&mut vec![0, 3, 0, 1, -3]), 10);
    }
}
