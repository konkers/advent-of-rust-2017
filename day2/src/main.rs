use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// This does not work because of the function pointer.
// #[derive(Debug)]
struct Checksummer {
    sum: u64,
    reduce_row: fn(data: &Vec<u64>) -> u64,
}

impl Checksummer {
    fn process_row(&mut self, data: &Vec<u64>) {
        self.sum += (self.reduce_row)(data);
    }
}

fn reduce_part1(data: &Vec<u64>) -> u64 {
    let mut min = std::u64::MAX;
    let mut max = std::u64::MIN;

    for val in data {
        if *val < min {
            min = *val;
        }
        if *val > max {
            max = *val;
        }
    }
    max - min
}

fn reduce_part2(data: &Vec<u64>) -> u64 {
    for i in 0..(data.len()-1) {
        let d1 = data[i];
        for j in (i+1)..data.len() {
            let d2 = data[j];
            if d1 % d2 == 0 {
                return d1 / d2;
            }
            if d2 % d1 == 0 {
                return d2 / d1;
            }
        }
    }
    0
}

fn handle_file(filename: &String) {
    let f = File::open(filename).expect("file not found.");
    let reader = BufReader::new(f);

    let mut csum1 = Checksummer {sum: 0, reduce_row: reduce_part1};
    let mut csum2 = Checksummer {sum: 0, reduce_row: reduce_part2};
    for line in reader.lines() {
        let linestr = line.unwrap();
        let vals: Vec<u64> = linestr.split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        csum1.process_row(&vals);
        csum2.process_row(&vals);
    }
    println!("{} checksum part 1: {}", filename, csum1.sum);
    println!("{} checksum part 2: {}", filename, csum2.sum);
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
        let mut csum = Checksummer {sum: 0, reduce_row: reduce_part1};
        csum.process_row(&vec![5, 1, 9, 5]);
        csum.process_row(&vec![7, 5, 3]);
        csum.process_row(&vec![2, 4, 6, 8]);
        assert_eq!(csum.sum, 18);
    }

    #[test]
    fn example_part2() {
        let mut csum = Checksummer {sum: 0, reduce_row: reduce_part2};
        csum.process_row(&vec![5, 9, 2, 8]);
        csum.process_row(&vec![9, 4, 7, 3]);
        csum.process_row(&vec![3, 8, 6, 5]);
        assert_eq!(csum.sum, 9);
    }
}
