use std::{io, result};
use std::collections::{BTreeSet, BTreeMap, VecDeque};
use std::ops::Deref;


fn gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        if b == 0 {
            return a;
        }
        let rem = a % b;
        a = b;
        b = rem;
    }
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(2, 3), 1);
    assert_eq!(gcd(100, 10), 10);
    assert_eq!(gcd(18, 12), 6);
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
struct Vec2i(i32, i32);

impl Vec2i {
    fn normalize(&self) -> Vec2i {
        if self.0 == 0 && self.1 == 0 {
            return *self
        }
        let diviser = gcd(self.0.abs(), self.1.abs());
        Vec2i(self.0 / diviser, self.1 / diviser)
    }
}

impl core::ops::Sub for Vec2i {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2i(self.0 - other.0, self.1 - other.1)
    }
}

impl core::ops::Add for Vec2i {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2i(self.0 + other.0, self.1 + other.1)
    }
}

fn visible_asteroids_from(center: Vec2i, others: &[Vec2i]) -> i32 {
    BTreeSet::<Vec2i>::from_iter(others.iter().filter(|x| **x != center).map(|x| (*x - center).normalize()))
        .len() as i32
}

fn calculate_nth_destroy_asteroid(center: Vec2i, others: &[Vec2i], nth: usize) -> Vec2i {
    let mut targets: BTreeMap<Vec2i, Vec<Vec2i>> = BTreeMap::new();
    for other in others {
        if *other != center {
            let diff = *other - center;
            let dir = diff.normalize();
            targets.entry(dir).or_default().push(diff);
        }
    }

    let mut targets_vec: Vec<(Vec2i, Vec<Vec2i>)> = targets.into_iter().collect();
    targets_vec.sort_by_key(|(dir, _)| {
        let (x, y) = (dir.0, -dir.1);
        let res = if x >= 0 && y >= 0 {
            - (y as f32).atan2(x as f32) - 100.0
        } else if y < 0 {
            - (y as f32).atan2(x as f32) - 50.0
        } else {
            - (y as f32).atan2(x as f32)
        };
        // hack: f32 does not impl Ord
        (res * 1000000.0) as i64
    });
    for target in targets_vec.iter_mut() {
        // sort in reverse, because we would pop it from back
        target.1.sort_by_key(|x| -(x.0 * x.0 + x.1 * x.1))
    }

    let mut results: Vec<Vec2i> = Vec::new();
    while !targets_vec.is_empty() {
        for item in targets_vec.iter_mut() {
            results.push(item.1.pop().unwrap());
        }
        targets_vec = targets_vec.into_iter().filter(|x| !x.1.is_empty()).collect();
    }

    return results[nth] + center;
}

fn parse_asteroids_map<S: Deref<Target = str>, T: Iterator<Item = S>>(input_map: T) -> Vec<Vec2i> {
    let mut result : Vec<Vec2i> = Vec::new();
    for (y, line) in input_map.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                result.push(Vec2i(x as i32, y as i32));
            }
        }
    }
    return result;
}

fn solve_a(map: &[Vec2i]) -> i32 {
    map.iter().map(|center| visible_asteroids_from(*center, map)).max().unwrap()
}

fn solve_b(map: &[Vec2i]) -> i32 {
    let center = map.iter()
        .map(|center| (*center, visible_asteroids_from(*center, map)))
        .max_by_key(|(_, v)| *v)
        .unwrap().0;
    let target = calculate_nth_destroy_asteroid(center, map, 199);
    target.0 * 100 + target.1
}

#[test]
fn example_test() {
    {
        let map = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        assert_eq!(solve_a(&parse_asteroids_map(map.split("\n"))), 33);
    }
    {
        let map = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(solve_b(&parse_asteroids_map(map.split("\n"))), 802);
    }
}


fn main() {
    let lines = io::stdin().lines();
    let map = parse_asteroids_map(lines.map(|x| x.unwrap()));
    println!("{}", solve_a(&map));
    println!("{}", solve_b(&map));
}
