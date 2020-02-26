use std::io::prelude::*;
use std::str::FromStr;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

struct CableStep {
    dir: Direction,
    len: i32,
}

impl FromStr for CableStep {
    type Err = ();

    fn from_str(s: &str) -> Result<CableStep, ()> {
        let d = Direction::from_str(&s[0..1])?;
        let l = match s[1..].parse::<i32>() {
            Ok(n) => Ok(n),
            _ => Err(()),
        }?;

        Ok(CableStep { dir: d, len: l })
    }
}

fn parse_steps(line: &str) -> Result<Vec<CableStep>, ()> {
    line.split(",").map(CableStep::from_str).collect()
}

#[derive(Clone, PartialEq, Debug)]
struct Coord2d {
    x: i32,
    y: i32,
}

fn offset_from_step(step: &CableStep) -> Coord2d {
    match step.dir {
        Direction::Left => Coord2d { x: -1, y: 0 },
        Direction::Right => Coord2d { x: 1, y: 0 },
        Direction::Up => Coord2d { x: 0, y: -1 },
        Direction::Down => Coord2d { x: 0, y: 1 },
    }
}

fn build_coords(steps: &Vec<CableStep>, start: Coord2d) -> Vec<Coord2d> {
    let mut coords = Vec::new();
    let mut last = start.clone();
    coords.push(start);
    for step in steps {
        let offset = offset_from_step(&step);
        for _ in 1..step.len + 1 {
            last = Coord2d {
                x: last.x + offset.x,
                y: last.y + offset.y,
            };
            coords.push(last.clone());
        }
    }

    coords
}

fn manhatten(c: &Coord2d) -> i32 {
    return c.x.abs() + c.y.abs();
}

fn matches(a: &Vec<Coord2d>, b: &Vec<Coord2d>) -> Vec<(usize, Coord2d)> {
    let mut intersections = Vec::new();
    let mut count = 0;
    let size = a.len();
    for (ai, aa) in a.iter().enumerate() {
        println!("{}  of {}", count, size);
        count += 1;
        if aa.x == 0 && aa.y == 0 {
            continue;
        }
        for (bi, bb) in b.iter().enumerate() {
            if aa == bb {
                intersections.push((ai + bi, aa.clone()));
            }
        }
    }
    intersections
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Provide one argument with path to the program");
    }

    let file = std::fs::File::open(&args[1]).unwrap();
    let reader = std::io::BufReader::new(file);
    let input_data: Vec<Vec<Coord2d>> = reader
        .lines()
        .map(|line| parse_steps(&line.unwrap()).unwrap())
        .map(|steps| build_coords(&steps, Coord2d { x: 0, y: 0 }))
        .collect();

    let m = matches(&input_data[0], &input_data[1]);
    println!("{:?}", m);
    println!(
        "Min by coord{:?}",
        m.iter()
            .map(|(_, coord)| manhatten(coord))
            .min()
            .unwrap()
    );
    println!(
        "Min by length {:?}",
        m.iter()
            .map(|(len, _)| len)
            .min()
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use build_coords;
    use manhatten;
    use matches;
    use parse_steps;
    use Coord2d;

    #[test]
    fn test1() {
        let test_str1 = "R8,U5,L5,D3";
        let test_str2 = "U7,R6,D4,L4";
        let steps1 = parse_steps(&test_str1).unwrap();
        let coords1 = build_coords(&steps1, Coord2d { x: 0, y: 0 });
        let steps2 = parse_steps(&test_str2).unwrap();
        let coords2 = build_coords(&steps2, Coord2d { x: 0, y: 0 });

        let m = matches(&coords1, &coords2);
        println!("{:?}", m);
        let themin = matches(&coords1, &coords2)
            .into_iter()
            .map(manhatten)
            .min()
            .unwrap();
        println!("{:?}", themin);
        assert_eq!(themin, 6);
    }
    //#[test]
    //fn test2() {
    //    let test_str1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83";
    //    let test_str2 =
    //        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51,U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    //    let steps1 = parse_steps(&test_str1).unwrap();
    //    let coords1 = build_coords(&steps1, Coord2d { x: 0, y: 0 });
    //    let steps2 = parse_steps(&test_str2).unwrap();
    //    let coords2 = build_coords(&steps2, Coord2d { x: 0, y: 0 });

    //    let m = matches(&coords1, &coords2);
    //    println!("{:?}", m);
    //    assert_eq!(m, vec![]);
    //}

}
