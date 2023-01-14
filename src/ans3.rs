use std::collections::HashMap;
use std::io;

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

type Motion = (Direction, i32);

type Coord = (i32, i32);

type PointInfo = HashMap<usize, i32>;  // path id -> step

struct Grid {
    grid: HashMap<Coord, PointInfo>,
    next_path_id: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Grid { grid: HashMap::new(), next_path_id: 0 }
    }
}

impl Grid {
    fn find_closest_intersect_distance(&self) -> i32 {
        self.grid.iter()
            .filter(|(_, info)| info.len() > 1)
            .map(|(coord, _)| i32::abs(coord.0) + i32::abs(coord.1))
            .min()
            .unwrap_or(0)
    }

    fn find_smallest_intersect_steps(&self) -> i32 {
        self.grid.iter()
            .filter(|(_, lines)| lines.len() > 1)
            .map(|(_, info)| info.values().sum())
            .min()
            .unwrap_or(0)
    }

    fn add_path(&mut self, motions: &[Motion]) {
        self.next_path_id += 1;

        let mut total_step = 0;
        let mut pos = (0, 0);
        for (dir, steps) in motions {
            let unit = match dir {
                Direction::Left => (-1, 0),
                Direction::Down => (0, -1),
                Direction::Right => (1, 0),
                Direction::Up => (0, 1),
            };
            for _ in 0..(*steps) {
                pos.0 += unit.0;
                pos.1 += unit.1;
                total_step += 1;
                self.grid.entry(pos).or_default()
                    .entry(self.next_path_id).or_insert(total_step);
            }
        }
    }
}


fn parse_motions(line: &str) -> Vec<Motion> {
    line.trim_end().split(",")
        .map(|s| (
            match s.chars().nth(0).unwrap() {
                'R' => Direction::Right,
                'L' => Direction::Left,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => panic!("invalid string: {}", s),
            },
            s[1..].parse().unwrap()
        ))
        .collect()
}

#[test]
fn test_0() {
    let mut grid = Grid::default();
    grid.add_path(&parse_motions("R8,U5,L5,D3"));
    grid.add_path(&parse_motions("U7,R6,D4,L4"));
    assert_eq!(grid.find_closest_intersect_distance(), 6);
}

#[test]
fn test_1() {
    let mut grid = Grid::default();
    grid.add_path(&parse_motions("R75,D30,R83,U83,L12,D49,R71,U7,L72"));
    grid.add_path(&parse_motions("U62,R66,U55,R34,D71,R55,D58,R83"));
    assert_eq!(grid.find_closest_intersect_distance(), 159);
    assert_eq!(grid.find_smallest_intersect_steps(), 610);
}

#[test]
fn test_2() {
    let mut grid = Grid::default();
    grid.add_path(&parse_motions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"));
    grid.add_path(&parse_motions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
    assert_eq!(grid.find_closest_intersect_distance(), 135);
    assert_eq!(grid.find_smallest_intersect_steps(), 410);
}

fn main() {
    let mut grid = Grid::default();
    for line in io::stdin().lines() {
        grid.add_path(&parse_motions(line.unwrap().trim_end()));
    }
    println!("smallest distance: {}", grid.find_closest_intersect_distance());
    println!("smallest step: {}", grid.find_smallest_intersect_steps());
}
