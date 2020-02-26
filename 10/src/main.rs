use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::Read;

fn gcd(a: i32, b: i32) -> i32 {
    let a_mag = i32::abs(a);
    let b_mag = i32::abs(b);
    if b_mag == 0 {
        a_mag
    } else {
        gcd(b_mag, a_mag % b_mag)
    }
}

fn wrap_angle(x: f32) -> f32 {
    let pos = if x >= 0.0 {
        x
    } else {
        x + std::f32::consts::PI * 2.0
    };
    let shift = pos - std::f32::consts::FRAC_PI_2;
    let shift2 = if shift >= 0.0 {
        shift % (std::f32::consts::PI * 2.0)
    } else {
        shift + std::f32::consts::PI * 2.0
    };
    (std::f32::consts::PI * 2.0 - shift2) % (std::f32::consts::PI * 2.0)
}

fn laser_sort(ast_map: &Vec<bool>, width: i32, height: i32, x: i32, y: i32) {
    let to_idx = |xx, yy| (xx + yy * width) as usize;

    #[derive(Clone, Copy, Debug)]
    struct Asteroid {
        coord: (i32, i32),
        dist_sq: i32,
        angle: f32,
    };

    let mut ast_by_angel_by_dist: Vec<Asteroid> = Vec::new();

    for yy in 0..height {
        for xx in 0..width {
            if (xx == x && yy == y) || !ast_map[to_idx(xx, yy)] {
                continue;
            }
            let mut x_dir = xx - x;
            let mut y_dir = yy - y;
            let dist_sq = x_dir * x_dir + y_dir * y_dir;

            let neg_pi_pi_angle: f32 = (-y_dir as f32).atan2(x_dir as f32);
            let phase_shifted_wrapped = wrap_angle(neg_pi_pi_angle);
            let a = Asteroid {
                coord: (xx, yy),
                dist_sq: dist_sq,
                angle: phase_shifted_wrapped,
            };

            ast_by_angel_by_dist.push(a);
        }
    }

    ast_by_angel_by_dist.sort_unstable_by(|a, b| {
        let angle_cmp = a.angle.partial_cmp(&b.angle).unwrap();
        if angle_cmp == Ordering::Equal {
            a.dist_sq.partial_cmp(&b.dist_sq).unwrap().reverse()
        } else {
            angle_cmp
        }
    });

    let mut new_res: Vec<Vec<Asteroid>> = Vec::new();
    while !ast_by_angel_by_dist.is_empty() {
        let first_angle = ast_by_angel_by_dist.first().unwrap().angle;
        let (same, diff): (Vec<Asteroid>, Vec<Asteroid>) = ast_by_angel_by_dist
            .iter()
            .partition(|x| x.angle == first_angle);
        new_res.push(same);
        ast_by_angel_by_dist = diff;
    }
    let mut new_res2: Vec<Asteroid> = Vec::new();

    while new_res2.len() < 352 {
        for v in &mut new_res {
            if v.is_empty() {
                continue;
            }
            new_res2.push(v.pop().unwrap());
        }
        eprintln!("{}", new_res2.len());
    }
    for l in &new_res2 {
        eprintln!("{:?}", l);
    }

    let result = new_res2.get(199).unwrap();
    // for i in &ast_by_angel_by_dist {
    //     eprintln!("{} {} {} ", i.coord.0, i.coord.1, {
    //         let x_dir = i.coord.0 - x;
    //         let y_dir = i.coord.1 - y;
    //         wrap_angle((y_dir as f32).atan2(x_dir as f32))
    //     });
    // }
    eprintln!(
        "200th: {} {} {}",
        result.coord.0,
        result.coord.1,
        result.coord.0 * 100 + result.coord.1
    );
}

fn score_fn(ast_map: &Vec<bool>, width: i32, height: i32, x: i32, y: i32) -> i32 {
    let to_idx = |xx, yy| (xx + yy * width) as usize;
    if !ast_map[to_idx(x, y)] {
        return 0;
    }

    let mut direction_set: HashSet<(i32, i32)> = HashSet::new();

    for yy in 0..height {
        for xx in 0..width {
            if (xx == x && yy == y) || !ast_map[to_idx(xx, yy)] {
                continue;
            }
            let mut x_dir = xx - x;
            let mut y_dir = yy - y;
            let gcd = gcd(x_dir, y_dir);
            x_dir = x_dir / gcd;
            y_dir = y_dir / gcd;

            direction_set.insert((x_dir, y_dir));
        }
    }

    direction_set.len() as i32
}

fn build_map(s: &String) -> (Vec<bool>, i32, i32) {
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    let mut map_builder: Vec<bool> = Vec::new();
    for l in s.lines() {
        if width == 0 {
            width = l.chars().count() as i32;
        }

        map_builder.extend(l.chars().map(|x| x == '#'));
        height += 1;
    }
    (map_builder, width, height)
}

fn find_max_score(map: &Vec<bool>, width: i32, height: i32) -> (i32, i32) {
    let mut scores: Vec<i32> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            scores.push(score_fn(&map, width, height, x, y));
        }
    }

    scores
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(i, v)| (i as i32, *v))
        .unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Provide three arguments with path to the input");
    }

    let file = std::fs::File::open(&args[1]).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let file_string = {
        let mut string = String::new();
        reader
            .read_to_string(&mut string)
            .expect("Unable to read file");

        string
    };

    let (ast_map, width, height) = build_map(&file_string);

    let (max_idx, max_count) = find_max_score(&ast_map, width, height);

    println!(
        "Max index {} {} {} {}",
        max_idx,
        max_count,
        max_idx % width,
        max_idx / width
    );
    laser_sort(&ast_map, width, height, max_idx % width, max_idx / width);
}

#[cfg(test)]
mod tests {

    use build_map;
    use find_max_score;

    #[test]
    fn test1() {
        let test = r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#;

        let (m, w, h) = build_map(&test.to_string());
        let (max_idx, max_count) = find_max_score(&m, w, h);

        eprintln!("{}, {} {}", max_idx, max_idx % w, max_idx / w);
        assert_eq!(max_idx, 5 + 8 * w);
        assert_eq!(max_idx % w, 5);
        assert_eq!(max_idx / w, 8);
    }
    #[test]
    fn test2() {
        let test = r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#;

        let (m, w, h) = build_map(&test.to_string());
        let (max_idx, max_count) = find_max_score(&m, w, h);

        eprintln!("{}, {} {}", max_idx, max_idx % w, max_idx / w);
        assert_eq!(max_idx, 1 + 2 * w);
        assert_eq!(max_idx % w, 1);
        assert_eq!(max_idx / w, 2);
    }
    #[test]
    fn test3() {
        let test = r#".#..##.###...#######
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
###.##.####.##.#..##"#;

        let (m, w, h) = build_map(&test.to_string());
        let (max_idx, max_count) = find_max_score(&m, w, h);

        eprintln!("{}, {} {}", max_idx, max_idx % w, max_idx / w);
        assert_eq!(max_idx, 11 + 13 * w);
        assert_eq!(max_idx % w, 11);
        assert_eq!(max_idx / w, 13);
    }
    #[test]
    fn test4() {
        let test = r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#;

        let (m, w, h) = build_map(&test.to_string());
        let (max_idx, max_count) = find_max_score(&m, w, h);

        eprintln!("{}, {} {}", max_idx, max_idx % w, max_idx / w);
        assert_eq!(max_idx, 6 + 3 * w);
        assert_eq!(max_idx % w, 6);
        assert_eq!(max_idx / w, 3);
    }
}
