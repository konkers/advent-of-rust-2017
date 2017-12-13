
#[derive(Debug)]
struct Point2d {
    x: i32,
    y: i32,
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

fn manhattan_dist(p1: Point2d, p2: Point2d) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn dist(position: i32) -> i32 {
    manhattan_dist(Point2d{x: 0, y: 0}, calc_location(position))
}

fn main() {
    let loc = 312051;
    println!("{}: {}", loc, dist(loc));
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
}
