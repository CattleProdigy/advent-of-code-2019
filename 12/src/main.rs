use std::collections::HashSet;
use std::io::Read;
extern crate nalgebra as na;
extern crate regex;
use regex::Regex;
type Vec3i = na::Vector3<i32>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Moon {
    pos: Vec3i,
    vel: Vec3i,
}

fn parse_moons_pos(file_string: &String) -> Vec<Vec3i> {
    let re = Regex::new(r"x=(-?[0-9]+).*y=(-?[0-9]+).*z=(-?[0-9]+)").unwrap();
    file_string
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let z = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            Vec3i::new(x, y, z)
        })
        .collect()
}

//
//

fn simulate_moon_step(moons: &Vec<Moon>) -> Vec<Moon> {
    // Get velocity updates
    let mut vel_updates = vec![Vec3i::new(0, 0, 0); moons.len()];
    for (i, m1) in moons.iter().enumerate() {
        for (j, m2) in moons.iter().enumerate().skip(i + 1) {
            let m1_vel_update = m1.pos.zip_map(&m2.pos, |p1, p2| match p1.cmp(&p2) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => -1,
            });
            let m2_vel_update = -1 * m1_vel_update;
            vel_updates[i] += m1_vel_update;
            vel_updates[j] += m2_vel_update;
        }
    }

    // apply velocity updates
    let mut new_moons = moons.to_vec();
    for (mut m, vu) in new_moons.iter_mut().zip(vel_updates.iter()) {
        m.vel += vu;
    }

    // apply position updates
    for mut m in new_moons.iter_mut() {
        m.pos += m.vel;
    }

    new_moons
}

fn energy(m: &Moon) -> i32 {
    let apos = m.pos.abs();
    let pot_eng = apos.x + apos.y + apos.z;
    let avel = m.vel.abs();
    let kin_eng = avel.x + avel.y + avel.z;
    pot_eng * kin_eng
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Provide one argument with path to the program");
    }

    let file = std::fs::File::open(&args[1]).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut file_string = String::new();
    reader
        .read_to_string(&mut file_string)
        .expect("Unable to read file");

    let moon_pos = parse_moons_pos(&file_string);
    {
        let mut moons = moon_pos
            .iter()
            .map(|pos| Moon {
                pos: *pos,
                vel: Vec3i::new(0, 0, 0),
            })
            .collect::<Vec<Moon>>();
        for _ in 0..1000 {
            moons = simulate_moon_step(&moons);
        }
        let total_energy: i32 = moons.iter().map(energy).sum();
        println!("Total NRG: {}", total_energy);
    }

    {
        let mut steps = Vec3i::new(0, 0, 0);
        for i in 0..3 {
            let mut moons = moon_pos
                .iter()
                .map(|pos| Moon {
                    pos: *pos,
                    vel: Vec3i::new(0, 0, 0),
                })
                .collect::<Vec<Moon>>();

            steps[i] = {
                let mut universe_states: HashSet<Vec<(i32, i32)>> = HashSet::new();
                let mut steps: i32 = 0;
                loop {
                    let mut xs: Vec<(i32, i32)> =
                        moons.iter().map(|m| (m.pos[i], m.vel[i])).collect();
                    if universe_states.contains(&xs) {
                        break;
                    }
                    universe_states.insert(xs);

                    moons = simulate_moon_step(&moons);
                    steps += 1;
                }
                println!("{}", steps);
                steps
            };
        }
        println!("x_steps: {}", steps);
    }

    // run_robot(program.to_vec());
}

#[cfg(test)]
mod tests {
    use parse_moons_pos;
    use simulate_moon_step;
    use Moon;
    use Vec3i;

    #[test]
    fn test1() {
        let test_input = r"<x=-8, y=-10, z=0>
                           <x=5, y=5, z=10>
                           <x=2, y=-7, z=3>
                           <x=9, y=-8, z=-3>";
        let moons = parse_moons_pos(&test_input.to_string());
        assert_eq!(moons.len(), 4);
        assert_eq!(moons[0], Vec3i::new(-8, -10, 0));
        assert_eq!(moons[1], Vec3i::new(5, 5, 10));
        assert_eq!(moons[2], Vec3i::new(2, -7, 3));
        assert_eq!(moons[3], Vec3i::new(9, -8, -3));
    }

    #[test]
    fn test2() {
        let test_input = r"<x=-1, y=0, z=2>
                           <x=2, y=-10, z=-7>
                           <x=4, y=-8, z=8>
                           <x=3, y=5, z=-1>";

        let moon_pos = parse_moons_pos(&test_input.to_string());
        let moons = moon_pos
            .iter()
            .map(|pos| Moon {
                pos: *pos,
                vel: Vec3i::new(0, 0, 0),
            })
            .collect::<Vec<Moon>>();
        let new_moons = simulate_moon_step(&moons);
        assert_eq!(new_moons.len(), 4);
        assert_eq!(new_moons[0].vel, Vec3i::new(3, -1, -1));
        assert_eq!(new_moons[1].vel, Vec3i::new(1, 3, 3));
        assert_eq!(new_moons[2].vel, Vec3i::new(-3, 1, -3));
        assert_eq!(new_moons[3].vel, Vec3i::new(-1, -3, 1));
        assert_eq!(new_moons[0].pos, Vec3i::new(2, -1, 1));
        assert_eq!(new_moons[1].pos, Vec3i::new(3, -7, -4));
        assert_eq!(new_moons[2].pos, Vec3i::new(1, -7, 5));
        assert_eq!(new_moons[3].pos, Vec3i::new(2, 2, 0));
    }
}
