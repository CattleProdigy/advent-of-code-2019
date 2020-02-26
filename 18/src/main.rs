use std::io::{stdin, stdout, BufRead, Read, Write};
extern crate nalgebra as na;
type Vec2i = na::Vector2<i32>;
type Vec2u = na::Vector2<usize>;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::iter;
use std::iter::FromIterator;

struct Map2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}
impl<T: Clone + Default> Map2D<T> {
    fn new(w: usize, h: usize) -> Map2D<T> {
        Map2D::new_fill(w, h, T::default())
    }
}
impl<T: Clone> Map2D<T> {
    fn new_fill(w: usize, h: usize, fill_val: T) -> Map2D<T> {
        let mut tmp_data: Vec<T> = Vec::new();
        tmp_data.resize(w * h, fill_val);
        Map2D::<T> {
            data: tmp_data,
            width: w,
            height: h,
        }
    }

    fn ind(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn ind_to_xy(&self, ind: usize) -> Vec2u {
        let y = ind / self.width;
        Vec2u::new(ind - y * self.width, y)
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        let i = self.ind(x,y);
        &mut self.data[i]
    }
    fn at(&self, x: usize, y: usize) -> &T {
        &self.data[self.ind(x, y)]
    }
    fn in_range(&self, xy: &Vec2u) -> bool {
        return xy.x < self.width && xy.y < self.height;
    }
    fn in_range_sign(&self, xy: &Vec2i) -> bool {
        return xy.x < self.width as i32 && xy.y < self.height as i32 && xy.x >= 0 && xy.y >= 0;
    }
}

impl<T: fmt::Display + Clone + Default> fmt::Display for Map2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&self.at(x, y).to_string());
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

fn parse_map(input: &String) -> Map2D<char> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut map: Map2D<char> = Map2D::new_fill(width, height, '#');

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            *(map.at_mut(x, y)) = c;
        }
    }

    map
}

fn shortest_path(
    map: &Map2D<char>,
    start: Vec2u,
    goal: char,
    passible_chars: &Vec<char>,
) -> Option<Vec<Vec2u>> {
    let mut queue: VecDeque<Vec2u> = VecDeque::new();
    let mut visited: Map2D<bool> = Map2D::new_fill(map.width, map.height, false);
    queue.push_back(start);
    let mut pred: Map2D<Vec2u> = Map2D::new_fill(map.width, map.height, Vec2u::new(0, 0));

    let mut finished: Option<Vec2u> = None;
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();

        if *map.at(cur.x, cur.y) == goal {
            finished = Some(cur);
            break;
        }

        let neighs: [Vec2u; 4] = [
            cur - Vec2u::new(1, 0),
            cur + Vec2u::new(1, 0),
            cur - Vec2u::new(0, 1),
            cur + Vec2u::new(0, 1),
        ];
        for n in neighs.iter() {
            let i = map.ind(n.x,n.y);
            if !map.in_range(n) {
                continue;
            }

            if visited.data[i] {
                continue;
            }
            let neigh_val = map.data[i];
            if passible_chars.iter().any(|x| neigh_val == *x) || neigh_val == goal {
                visited.data[i] = true;
                pred.data[i] = cur;
                queue.push_back(*n);
            }
        }
    }

    match finished {
        Some(x) => {
            let mut traj: Vec<Vec2u> = Vec::new();
            let mut backtrace = x;
            while backtrace != start {
                traj.push(backtrace);
                backtrace = *pred.at(backtrace.x, backtrace.y);
            }
            traj.push(start);
            traj.reverse();

            Some(traj)
        }
        None => None,
    }
}

fn key_paths_impl(
    map: &Map2D<char>,
    start: Vec2u,
    needed_keys: Vec<char>,
    keys: Vec<char>,
    traj_so_far: Vec<Vec2u>,
    trajs: &mut Vec<Vec2u>,
) {
    if needed_keys.is_empty() {
        if trajs.is_empty() || (traj_so_far.len() < trajs.len()) {
            eprintln!("found it {}", traj_so_far.len());
            *trajs = traj_so_far.to_vec();
        }
        return;
    }

    if !trajs.is_empty() && (traj_so_far.len() >= trajs.len()) {
        return;
    }

    let passible_chars = keys
        .iter()
        .map(|x| *x)
        .chain(keys.iter().map(|x| (*x).to_ascii_uppercase()))
        .chain(iter::once('.'))
        .chain(iter::once('@'))
        .collect::<Vec<_>>();
    // eprintln!("s:{}", passible_chars.len());

    for nk in needed_keys.iter() {
        //eprintln!("trying for {}", nk);
        let sp = shortest_path(&map, start, *nk, &passible_chars);
        match sp {
            Some(x) => {
                //eprintln!("found {}", nk);
                //if !trajs.is_empty() && (traj_so_far.len() + x.len()) >= trajs.len() {
                //    continue;
                //}
                let mut new_traj = traj_so_far.to_vec();
                new_traj.append(&mut x.to_vec());

                let mut new_keys = keys.clone();
                new_keys.push(*nk);
                let mut new_needed_keys = needed_keys
                    .iter()
                    .cloned()
                    .filter(|x| x != nk)
                    .collect::<Vec<_>>();
                let new_start = x.last().unwrap();
                key_paths_impl(map, *new_start, new_needed_keys, new_keys, new_traj, trajs);
            }
            None => {
                //eprintln!("not found {}", nk);
            }
        }
    }
}

fn key_paths(map: &Map2D<char>, start: Vec2u, needed_keys: Vec<char>) -> Vec<Vec2u> {
    let mut trajs: Vec<Vec2u> = Vec::new();

    key_paths_impl(map, start, needed_keys, Vec::new(), Vec::new(), &mut trajs);

    trajs
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

    let map = parse_map(&file_string);

    let start: Vec2u = map.ind_to_xy(
        map.data
            .iter()
            .enumerate()
            .find(|(_, &x)| x == '@')
            .map(|(i, _)| i)
            .unwrap(),
    );

    let keys: Vec<(char, Vec2u)> = map
        .data
        .iter()
        .enumerate()
        .filter(|(_, &x)| x.is_ascii_lowercase())
        .map(|(i, &x)| (x, map.ind_to_xy(i)))
        .collect::<Vec<_>>();

    let doors: Vec<(char, Vec2u)> = map
        .data
        .iter()
        .enumerate()
        .filter(|(_, &x)| x.is_ascii_uppercase())
        .map(|(i, &x)| (x, map.ind_to_xy(i)))
        .collect::<Vec<_>>();
    eprintln!("Map {}", map);
    eprintln!("Start {}", start);
    eprintln!("Keys {:?}", keys);
    eprintln!("Doors {:?}", doors);

    // for (k, loc) in keys {
    //     let sp = shortest_path(&map, start, k, &vec!['.']);
    //     match sp {
    //         Some(x) => {
    //             eprintln!("Sp to {}, \n{:?}", k, x);
    //         }
    //         None => {
    //             eprintln!("Couldn't find {}", k);
    //         }
    //     }
    // }

    let paths = key_paths(
        &map,
        start,
        keys.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
    );
    if !paths.is_empty() {
        eprintln!("Paths:\n {:?}", paths[0]);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}

    #[test]
    fn test2() {}
}
