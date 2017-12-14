use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug)]
#[derive(Eq)]
struct Point2d {
    x: i32,
    y: i32,
}

impl Ord for Point2d {
    fn cmp(&self, other: &Point2d) -> Ordering {
        if self.x < other.x {
            return Ordering::Less;
        } else if self.x > other.x {
            return Ordering::Greater;
        } else {
            if self.y < other.y {
                return Ordering::Less;
            } else if self.y > other.y {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for Point2d {
    fn partial_cmp(&self, other: &Point2d) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point2d {
    fn eq(&self, other: &Point2d) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct SpiralMem {
    // Probably not the best way to represent the "spiral" memory but
    // it gives me the opportunity to implement support for a BTreeMap
    // with a custom key.
    mem: BTreeMap<Point2d, i32>,
}

impl SpiralMem {
    pub fn new() -> SpiralMem {
        let mut mem = SpiralMem {
            mem: BTreeMap::new(),
        };
        mem.set(Point2d{x:0, y:0}, 1);
        mem
    }

    pub fn set(&mut self, loc: Point2d, val: i32) {
        self.mem.insert(loc, val);
    }

    pub fn get(&self, loc: Point2d) -> i32 {
        match self.mem.get(&loc) {
            Some(&val) => val,
            None => 0,
        }
    }
}

fn calc_location(i: i32) -> Point2d {
    if i <= 1 {
        return Point2d{x:0, y:0};
    }

    let sq = ((i-1) as f64).sqrt();
    let ring = ((sq+1.0) as i32) / 2;
    let ring_size = ring * 2 + 1;
    let first_num = (ring_size - 2) * (ring_size - 2) + 1;
    let quadrant = (i - first_num) / (ring_size - 1);
    let offset = (i - first_num) % (ring_size - 1);

    if quadrant == 0 {
        return Point2d{x: ring, y: -(ring - 1) + offset}
    } else if quadrant == 1 {
        return Point2d{x: ring - 1 - offset, y: ring}
    } else if quadrant == 2 {
        return Point2d{x: -ring, y: ring - 1 - offset}
    } else {
        return Point2d{x:  -(ring - 1) + offset, y: -ring}
    }
}

fn find_greater(number: i32) -> i32 {
    if number < 1 {
        return 1;
    }

    let mut i = 2;
    let mut mem = SpiralMem::new();

    loop {
        let loc = calc_location(i);
        let val =
            mem.get(Point2d{x: loc.x - 1, y: loc.y - 1}) +
            mem.get(Point2d{x: loc.x, y: loc.y - 1}) +
            mem.get(Point2d{x: loc.x + 1, y: loc.y - 1}) +

            mem.get(Point2d{x: loc.x - 1, y: loc.y}) +
            mem.get(Point2d{x: loc.x + 1, y: loc.y}) +

            mem.get(Point2d{x: loc.x - 1, y: loc.y + 1}) +
            mem.get(Point2d{x: loc.x, y: loc.y + 1}) +
            mem.get(Point2d{x: loc.x + 1, y: loc.y + 1});

        mem.set(loc, val);

        if val > number {
            return val;
        }
        i += 1;
    }
}

fn manhattan_dist(p1: Point2d, p2: Point2d) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn dist(position: i32) -> i32 {
    manhattan_dist(Point2d{x: 0, y: 0}, calc_location(position))
}

fn main() {
    let input = 312051;
    println!("dist {}: {}", input, dist(input));
    println!("first val > {}: {}", input, find_greater(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(dist(1), 0);
        assert_eq!(dist(12), 3);
        assert_eq!(dist(23), 2);
        assert_eq!(dist(1024), 31);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(find_greater(0), 1);
        assert_eq!(find_greater(1), 2);
        assert_eq!(find_greater(2), 4);
        assert_eq!(find_greater(3), 4);
        assert_eq!(find_greater(4), 5);
        assert_eq!(find_greater(5), 10);
        assert_eq!(find_greater(6), 10);
        assert_eq!(find_greater(9), 10);
        assert_eq!(find_greater(10), 11);
        assert_eq!(find_greater(11), 23);
        assert_eq!(find_greater(12), 23);
        assert_eq!(find_greater(805), 806);
    }
}
