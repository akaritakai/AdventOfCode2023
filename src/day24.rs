use crate::puzzle::Puzzle;
use itertools::Itertools;
use lazy_regex::regex;
use rayon::prelude::*;
use std::ops::{Add, Mul};
use z3::ast::{Ast, Int, Real};
use z3::{Context, Solver};

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let hailstones = self.parse_hailstones();
        count_intersections::<200_000_000_000_000, 400_000_000_000_000>(&hailstones).to_string()
    }

    fn solve_part_2(&self) -> String {
        let hailstones = self.parse_hailstones();
        let ctx = &Context::new(&z3::Config::default());
        let hailstone = find_missing_hailstone(ctx, &hailstones);
        (hailstone.0 + hailstone.1 + hailstone.2).to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }

    fn parse_hailstones(&self) -> Vec<Hailstone> {
        self.input
            .lines()
            .map(|line| {
                let mut cap = regex!(r"-?\d+").captures_iter(line);
                let px = cap.next().unwrap()[0].parse::<i64>().unwrap();
                let py = cap.next().unwrap()[0].parse::<i64>().unwrap();
                let pz = cap.next().unwrap()[0].parse::<i64>().unwrap();
                let vx = cap.next().unwrap()[0].parse::<i64>().unwrap();
                let vy = cap.next().unwrap()[0].parse::<i64>().unwrap();
                let vz = cap.next().unwrap()[0].parse::<i64>().unwrap();
                Hailstone::new(px, py, pz, vx, vy, vz)
            })
            .collect()
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vec3(i64, i64, i64);
#[derive(PartialEq, Clone, Copy, Debug)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

impl Hailstone {
    fn new(px: i64, py: i64, pz: i64, vx: i64, vy: i64, vz: i64) -> Self {
        Self {
            pos: Vec3(px, py, pz),
            vel: Vec3(vx, vy, vz),
        }
    }
}

fn count_intersections<const MIN: i64, const MAX: i64>(stones: &[Hailstone]) -> usize {
    stones
        .iter()
        .tuple_combinations()
        .par_bridge()
        .filter(|(stone1, stone2)| intersects::<MIN, MAX>(stone1, stone2))
        .count()
}

fn intersects<const MIN: i64, const MAX: i64>(stone1: &Hailstone, stone2: &Hailstone) -> bool {
    let (x1, y1, dx1, dy1) = (
        stone1.pos.0 as f64,
        stone1.pos.1 as f64,
        stone1.vel.0 as f64,
        stone1.vel.1 as f64,
    );
    let (x2, y2, dx2, dy2) = (
        stone2.pos.0 as f64,
        stone2.pos.1 as f64,
        stone2.vel.0 as f64,
        stone2.vel.1 as f64,
    );

    let m1 = dy1 / dx1;
    let m2 = dy2 / dx2;
    if (m2 - m1).abs() <= f64::EPSILON {
        return false;
    }
    let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
    let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);
    dx1.signum() == (x - x1).signum()
        && dx2.signum() == (x - x2).signum()
        && x >= MIN as f64
        && x <= MAX as f64
        && y >= MIN as f64
        && y <= MAX as f64
}

fn find_missing_hailstone(ctx: &Context, stones: &[Hailstone]) -> Vec3 {
    let solver = Solver::new(ctx);
    let px1 = Real::new_const(ctx, "px");
    let py1 = Real::new_const(ctx, "py");
    let pz1 = Real::new_const(ctx, "pz");
    let vx1 = Real::new_const(ctx, "vx");
    let vy1 = Real::new_const(ctx, "vy");
    let vz1 = Real::new_const(ctx, "vz");
    for (i, stone) in stones.iter().enumerate() {
        let t = Real::new_const(ctx, format!("t_{}", i));
        let fx1 = px1.clone().add(&vx1.clone().mul(&t));
        let fy1 = py1.clone().add(&vy1.clone().mul(&t));
        let fz1 = pz1.clone().add(&vz1.clone().mul(&t));
        let px2 = Real::from_int(&Int::from_i64(ctx, stone.pos.0));
        let py2 = Real::from_int(&Int::from_i64(ctx, stone.pos.1));
        let pz2 = Real::from_int(&Int::from_i64(ctx, stone.pos.2));
        let vx2 = Real::from_int(&Int::from_i64(ctx, stone.vel.0));
        let vy2 = Real::from_int(&Int::from_i64(ctx, stone.vel.1));
        let vz2 = Real::from_int(&Int::from_i64(ctx, stone.vel.2));
        let fx2 = px2.clone().add(&vx2.clone().mul(&t));
        let fy2 = py2.clone().add(&vy2.clone().mul(&t));
        let fz2 = pz2.clone().add(&vz2.clone().mul(&t));
        solver.assert(&fx1._eq(&fx2));
        solver.assert(&fy1._eq(&fy2));
        solver.assert(&fz1._eq(&fz2));
        solver.assert(&t.ge(&Real::from_int(&Int::from_i64(ctx, 0))));
    }
    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.eval(&px1, false).unwrap().as_real().unwrap().0;
    let y = model.eval(&py1, false).unwrap().as_real().unwrap().0;
    let z = model.eval(&pz1, false).unwrap().as_real().unwrap().0;
    Vec3(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use z3::Config;

    #[test]
    fn test_part_1_example_1() {
        let hailstones = vec![
            Hailstone::new(19, 13, 30, -2, 1, -2),
            Hailstone::new(18, 19, 22, -1, -1, -2),
            Hailstone::new(20, 25, 34, -2, -2, -4),
            Hailstone::new(12, 31, 28, -1, -2, -1),
            Hailstone::new(20, 19, 15, 1, -5, -3),
        ];
        assert_eq!(count_intersections::<7, 27>(&hailstones), 2);
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/24")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "18184");
    }

    #[test]
    fn test_part_2_example_1() {
        let hailstones = vec![
            Hailstone::new(19, 13, 30, -2, 1, -2),
            Hailstone::new(18, 19, 22, -1, -1, -2),
            Hailstone::new(20, 25, 34, -2, -2, -4),
            Hailstone::new(12, 31, 28, -1, -2, -1),
            Hailstone::new(20, 19, 15, 1, -5, -3),
        ];
        let ctx = &Context::new(&Config::default());
        assert_eq!(find_missing_hailstone(ctx, &hailstones), Vec3(24, 13, 10));
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/24")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "557789988450159");
    }
}
