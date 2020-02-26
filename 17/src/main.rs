use std::cell::RefCell;
use std::char;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::io::{stdin, stdout, BufRead, Read, Write};
extern crate nalgebra as na;
type Vec2i = na::Vector2<i32>;

#[derive(Copy, Clone, PartialEq)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpTrue = 5,
    JumpFalse = 6,
    LessThan = 7,
    Equals = 8,
    AdjustRelBase = 9,
    Halt = 99,
}

impl Opcode {
    fn from_i64(value: i64) -> Result<Opcode, String> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpTrue),
            6 => Ok(Opcode::JumpFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            9 => Ok(Opcode::AdjustRelBase),
            99 => Ok(Opcode::Halt),
            _ => Err(format!("Invalid Opcode: {}", value)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum OperandMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl OperandMode {
    fn from_i64(value: i64) -> Result<OperandMode, String> {
        match value {
            0 => Ok(OperandMode::Position),
            1 => Ok(OperandMode::Immediate),
            2 => Ok(OperandMode::Relative),
            _ => Err(format!("Invalid OperandMode: {}", value)),
        }
    }
}

fn operand_modes(x: i64) -> [OperandMode; 3] {
    let mut res: [OperandMode; 3] = [OperandMode::Position; 3];

    let mut x_div = x;
    res[0] = OperandMode::from_i64(x_div % 10).expect("");
    x_div = x_div / 10;
    res[1] = OperandMode::from_i64(x_div % 10).expect("");
    x_div = x_div / 10;
    res[2] = OperandMode::from_i64(x_div % 10).expect("");

    res
}

#[derive(Debug)]
enum ParsedInstruction {
    Add { op1: i64, op2: i64, dest: usize },
    Multiply { op1: i64, op2: i64, dest: usize },
    Input { dest: usize },
    Output { out: i64 },
    JumpTrue { test: i64, jump_dest: usize },
    JumpFalse { test: i64, jump_dest: usize },
    LessThan { op1: i64, op2: i64, dest: usize },
    Equals { op1: i64, op2: i64, dest: usize },
    AdjustRelBase { adj: i64 },
    Halt,
}

fn get_ext(v: &mut Vec<i64>, index: usize) -> i64 {
    if index >= v.len() {
        v.resize(index + 1, 0);
    }
    *v.get(index).unwrap()
}
fn get_mut_ext(v: &mut Vec<i64>, index: usize) -> &mut i64 {
    if index >= v.len() {
        v.resize(index + 1, 0);
    }
    v.get_mut(index).unwrap()
}

fn load_operands<'a>(
    op: Opcode,
    op_modes: [OperandMode; 3],
    ps: &mut ProcState,
    prog: &mut Vec<i64>,
) -> ParsedInstruction {
    let mut parse_operand = |i, read| -> i64 {
        let raw_opand_val = get_ext(prog, ps.prog_count as usize);
        let op = if read {
            match i {
                OperandMode::Position => get_ext(prog, raw_opand_val as usize),
                OperandMode::Immediate => raw_opand_val,
                OperandMode::Relative => get_ext(prog, (ps.rel_base + raw_opand_val) as usize),
            }
        } else {
            match i {
                OperandMode::Position => raw_opand_val,
                OperandMode::Immediate => raw_opand_val,
                OperandMode::Relative => ps.rel_base + raw_opand_val,
            }
        };
        ps.prog_count += 1;
        op
    };

    match op {
        Opcode::Add => ParsedInstruction::Add {
            op1: parse_operand(op_modes[0], true),
            op2: parse_operand(op_modes[1], true),
            dest: parse_operand(op_modes[2], false) as usize,
        },
        Opcode::Multiply => ParsedInstruction::Multiply {
            op1: parse_operand(op_modes[0], true),
            op2: parse_operand(op_modes[1], true),
            dest: parse_operand(op_modes[2], false) as usize,
        },
        Opcode::Input => ParsedInstruction::Input {
            dest: parse_operand(op_modes[0], false) as usize,
        },
        Opcode::Output => ParsedInstruction::Output {
            out: parse_operand(op_modes[0], true),
        },
        Opcode::JumpTrue => ParsedInstruction::JumpTrue {
            test: parse_operand(op_modes[0], true),
            jump_dest: parse_operand(op_modes[1], true) as usize,
        },
        Opcode::JumpFalse => ParsedInstruction::JumpFalse {
            test: parse_operand(op_modes[0], true),
            jump_dest: parse_operand(op_modes[1], true) as usize,
        },
        Opcode::LessThan => ParsedInstruction::LessThan {
            op1: parse_operand(op_modes[0], true),
            op2: parse_operand(op_modes[1], true),
            dest: parse_operand(op_modes[2], false) as usize,
        },
        Opcode::Equals => ParsedInstruction::Equals {
            op1: parse_operand(op_modes[0], true),
            op2: parse_operand(op_modes[1], true),
            dest: parse_operand(op_modes[2], false) as usize,
        },
        Opcode::AdjustRelBase => ParsedInstruction::AdjustRelBase {
            adj: parse_operand(op_modes[0], true) as i64,
        },
        Opcode::Halt => ParsedInstruction::Halt,
    }
}

fn parse_next_instr(ps: &mut ProcState, prog: &mut Vec<i64>) -> ParsedInstruction {
    let combined_opcode = get_ext(prog, ps.prog_count);
    ps.prog_count += 1;
    let opcode_int = combined_opcode % 100;
    let operand_modes_int = combined_opcode / 100;
    let opcode = Opcode::from_i64(opcode_int).unwrap();
    let operand_modes = operand_modes(operand_modes_int);

    load_operands(opcode, operand_modes, ps, prog)
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct ProcState {
    prog_count: usize,
    rel_base: i64,
    halted: bool,
}

fn process<I, O>(prog: &mut Vec<i64>, mut pc: ProcState, input: &mut I, output: &mut O) -> ProcState
where
    I: FnMut() -> Option<String>,
    O: FnMut(i64) -> (),
{
    loop {
        let cached_pc = pc;
        let instr = parse_next_instr(&mut pc, prog);
        // eprintln!("{:?}: {:?}", cached_pc, instr);

        match instr {
            ParsedInstruction::Add { op1, op2, dest } => {
                *get_mut_ext(prog, dest) = op1 + op2;
            }
            ParsedInstruction::Multiply { op1, op2, dest } => {
                *get_mut_ext(prog, dest) = op1 * op2;
            }
            ParsedInstruction::Input { dest } => match input() {
                Some(line) => {
                    //eprintln!("Input {} -> {}", line, dest);
                    let res = line.parse::<i64>().unwrap();
                    *get_mut_ext(prog, dest) = res;
                    if res == 0 {
                        pc.halted = true;
                    }
                }
                None => {
                    eprintln!("Breaking to wait for input");
                    pc = cached_pc;
                    break;
                }
            },
            ParsedInstruction::Output { out } => {
                output(out);
            }
            ParsedInstruction::JumpTrue { test, jump_dest } => {
                if test != 0 {
                    pc.prog_count = jump_dest;
                }
            }
            ParsedInstruction::JumpFalse { test, jump_dest } => {
                if test == 0 {
                    pc.prog_count = jump_dest;
                }
            }
            ParsedInstruction::LessThan { op1, op2, dest } => {
                *get_mut_ext(prog, dest) = if op1 < op2 { 1 } else { 0 };
            }

            ParsedInstruction::Equals { op1, op2, dest } => {
                *get_mut_ext(prog, dest) = if op1 == op2 { 1 } else { 0 };
            }

            ParsedInstruction::AdjustRelBase { adj } => {
                pc.rel_base += adj;
            }

            ParsedInstruction::Halt => {
                eprintln!("\nHALTING, press 'q' to exit");
                pc.halted = true;
                break;
            }
        }
    }

    pc
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Scaffold = 35,
    Open = 46,
    NewLine = 10,
    Tumbling = 88,
    RobUp = 94,
    RobDown = 118,
    RobLeft = 60,
    RobRight = 63,
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = (*self as i32) as u8 as char;
        write!(f, "{}", c)
    }
}

impl Tile {
    fn from_i64(value: i64) -> Result<Tile, String> {
        match value {
            10 => Ok(Tile::NewLine),
            118 => Ok(Tile::RobDown),
            35 => Ok(Tile::Scaffold),
            46 => Ok(Tile::Open),
            60 => Ok(Tile::RobLeft),
            63 => Ok(Tile::RobRight),
            88 => Ok(Tile::Tumbling),
            94 => Ok(Tile::RobUp),
            _ => Err(format!("Invalid Tile: {}", value)),
        }
    }
}

fn run_game(mut prog: Vec<i64>) -> HashMap<Vec2i, Tile> {
    struct ProgState {
        cur_readout: Vec2i,
        map: HashMap<Vec2i, Tile>,
    };
    let prog_state_ref: RefCell<ProgState> = RefCell::new(ProgState {
        cur_readout: Vec2i::new(0, 0),
        map: HashMap::new(),
    });

    let mut input = || {
        // let mut p = prog_state_ref.borrow_mut();

        Some("0".to_string())
    };

    //out
    let mut output = |x: i64| {
        let mut p = prog_state_ref.borrow_mut();
        let tile = Tile::from_i64(x).unwrap();
        if tile == Tile::NewLine {
            p.cur_readout.x = 0;
            p.cur_readout.y += 1;
        } else {
            let readout = p.cur_readout;
            p.map.insert(readout, tile);
            p.cur_readout.x += 1;
        }
    };
    let mut ps = ProcState {
        prog_count: 0,
        rel_base: 0,
        halted: false,
    };
    while !ps.halted {
        ps = process(&mut prog, ps, &mut input, &mut output);
    }

    let pb = prog_state_ref.borrow();
    pb.map.clone()
}

fn run_game_b(mut prog: Vec<i64>, commands: Vec<i64>) {
    struct ProgState {
        cur_readout: Vec2i,
        map: HashMap<Vec2i, Tile>,
    };
    let prog_state_ref: RefCell<ProgState> = RefCell::new(ProgState {
        cur_readout: Vec2i::new(0, 0),
        map: HashMap::new(),
    });

    let mut input_i = commands.iter();
    let mut input = || {
        // let mut p = prog_state_ref.borrow_mut();

        Some(input_i.next().unwrap().to_string())
    };

    //out
    let mut buffer_output = |x: i64| {
        print!("{}", char::from_u32(x as u32).unwrap_or(' '));
        eprint!("{}", x);
    };
    let mut ps = ProcState {
        prog_count: 0,
        rel_base: 0,
        halted: false,
    };
    while !ps.halted {
        ps = process(&mut prog, ps, &mut input, &mut buffer_output);
    }
}

fn run_with_stdin_stdout(mut prog: Vec<i64>) {
    let mut buffer_input = || {
        let out = {
            let stdin = std::io::stdin();
            let handle = stdin.lock();
            let res = handle.lines().next().unwrap();
            res
        };
        match out {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    };
    let mut buffer_output = |x: i64| {
        println!("OUT-> {}", x);
    };
    let ps = ProcState {
        prog_count: 0,
        rel_base: 0,
        halted: false,
    };
    process(&mut prog, ps, &mut buffer_input, &mut buffer_output);
}

fn canonical_edge(a: &Vec2i, b: &Vec2i) -> (Vec2i, Vec2i) {
    if a.x < b.x {
        (*a, *b)
    } else if a.x == b.x {
        if a.y < b.y {
            (*a, *b)
        } else {
            (*b, *a)
        }
    } else {
        (*b, *a)
    }
}

fn dfs_edges(
    vert: Vec2i,
    adj: &HashMap<Vec2i, Vec<Vec2i>>,
    tgt: usize,
    mut traj: Vec<Vec2i>,
    visited: HashMap<(Vec2i, Vec2i), i32>,
    traj_strs: &mut Vec<String>,
) {
    traj.push(vert);
    // eprintln!("traj: {}, vis: {}", traj.len(), visited.len());
    if visited.len() == tgt {
        // for v in traj.iter() {
        //     eprintln!("{} {}", v.x, v.y);
        // }

        let turn = |cur: Vec2i, new: Vec2i| -> Option<char> {
            let res = if cur.dot(&new) != 0 {
                None
            } else {
                if cur.x < 0 {
                    if new.y < 0 {
                        Some('R')
                    } else {
                        Some('L')
                    }
                } else if cur.x > 0 {
                    if new.y < 0 {
                        Some('L')
                    } else {
                        Some('R')
                    }
                } else if cur.y < 0 {
                    if new.x < 0 {
                        Some('L')
                    } else {
                        Some('R')
                    }
                } else {
                    if new.x < 0 {
                        Some('R')
                    } else {
                        Some('L')
                    }
                }
            };
            res
        };

        let mut traj_dir: String = String::new();
        let mut dir = Vec2i::new(0, -1);
        let mut cur = *traj.first().unwrap();
        let mut step_count: i32 = 0;
        for t in traj.iter().skip(1) {
            let new_dir = t - cur;
            let turn_char = turn(dir, new_dir);
            match turn_char {
                Some(x) => {
                    if step_count > 0 {
                        traj_dir.push_str(&step_count.to_string());
                    }
                    traj_dir.push(x);
                    dir = new_dir;
                    step_count = i32::abs(new_dir.x) + i32::abs(new_dir.y);
                }
                None => {
                    step_count += i32::abs(new_dir.x) + i32::abs(new_dir.y);
                }
            }

            cur = *t;
        }
        traj_dir.push_str(&step_count.to_string());
        traj_strs.push(traj_dir);

        return;
    }

    // eprintln!("at {} ", vert);
    let mut cedges = adj[&vert]
        .iter()
        .map(|x| canonical_edge(x, &vert))
        .collect::<Vec<_>>();
    cedges.sort_by(|a, b| {
        let a_score = match visited.get(a) {
            Some(x) => *x,
            None => 0,
        };
        let b_score = match visited.get(b) {
            Some(x) => *x,
            None => 0,
        };
        a_score.partial_cmp(&b_score).unwrap()
    });
    let mut c: i32 = 0;
    for ce in cedges {
        let zero: i32 = 0;
        if *visited.get(&ce).unwrap_or(&zero) >= 1 {
            continue;
        }
        c += 1;

        let mut new_visited = visited.clone();
        *new_visited.entry(ce).or_insert(0) += 1;
        let other_edge = if ce.0 == vert { ce.1 } else { ce.0 };
        dfs_edges(other_edge, adj, tgt, traj.to_vec(), new_visited, traj_strs)
    }
}

fn get_vertices(dims: Vec2i, map: &HashMap<Vec2i, Tile>) -> (Vec<String>, Vec<Vec2i>) {
    let mut vert: HashSet<Vec2i> = HashSet::new();
    for y in 0..dims.y {
        for x in 0..dims.x {
            let loc = Vec2i::new(x, y);

            if !map.contains_key(&loc) {
                continue;
            }
            let valid = |x: Tile| x == Tile::Scaffold || x == Tile::RobUp;
            if !valid(map[&loc]) {
                continue;
            }

            let left: bool = {
                let nloc = loc + Vec2i::new(-1, 0);
                map.contains_key(&nloc) && valid(map[&nloc])
            };
            let right: bool = {
                let nloc = loc + Vec2i::new(1, 0);
                map.contains_key(&nloc) && valid(map[&nloc])
            };
            let up: bool = {
                let nloc = loc + Vec2i::new(0, -1);
                map.contains_key(&nloc) && valid(map[&nloc])
            };
            let down: bool = {
                let nloc = loc + Vec2i::new(0, 1);
                map.contains_key(&nloc) && valid(map[&nloc])
            };

            let count: i32 = left as i32 + right as i32 + up as i32 + down as i32;

            let is_vert: bool = !(left && right && !up && !down)
                && !(up && down && !left && !right)
                || map[&loc] == Tile::RobUp
                || count == 1;

            if is_vert {
                vert.insert(loc);
            }
        }
    }

    let mut adj: HashMap<Vec2i, Vec<Vec2i>> = HashMap::new();
    for v in &vert {
        let neighs: [Vec2i; 4] = [
            Vec2i::new(-1, 0), // west
            Vec2i::new(1, 0),  // east
            Vec2i::new(0, -1), // north
            Vec2i::new(0, 1),  // south
        ];

        for n in neighs.iter() {
            let mut nloc = v + n;
            if !(map.contains_key(&nloc)) {
                continue;
            }
            if map[&nloc] != Tile::Scaffold {
                continue;
            }

            while !vert.contains(&nloc) {
                nloc = nloc + n;
                if !(map.contains_key(&nloc)) {
                    break;
                }
            }
            if vert.contains(&nloc) {
                adj.entry(*v).or_insert(Vec::new()).push(nloc);
            }
        }
    }

    let mut edges: HashSet<(Vec2i, Vec2i)> = HashSet::new();
    for (v, a) in &adj {
        for v2 in a {
            edges.insert(canonical_edge(&v, &v2));
        }
    }

    let start: Vec2i = map
        .iter()
        .find(|(_, &v)| v == Tile::RobUp)
        .unwrap()
        .0
        .clone();

    let mut traj_strs: Vec<String> = Vec::new();
    dfs_edges(
        start,
        &adj,
        edges.len(),
        Vec::new(),
        HashMap::new(),
        &mut traj_strs,
    );

    (traj_strs, vert.into_iter().collect::<Vec<_>>())
}

fn get_inters(dims: Vec2i, map: &HashMap<Vec2i, Tile>) -> Vec<Vec2i> {
    let mut res: Vec<Vec2i> = Vec::new();
    for y in 0..dims.y {
        for x in 0..dims.x {
            let loc = Vec2i::new(x, y);
            if !map.contains_key(&loc) {
                continue;
            }
            let neighs: [Vec2i; 4] = [
                Vec2i::new(-1, 0),
                Vec2i::new(1, 0),
                Vec2i::new(0, -1),
                Vec2i::new(0, 1),
            ];

            if map[&loc] != Tile::Scaffold {
                continue;
            }

            let inter = neighs.iter().all(|n| {
                let nloc = loc + n;
                if !map.contains_key(&nloc) {
                    false
                } else {
                    map[&nloc] == Tile::Scaffold
                }
            });
            if inter == true {
                res.push(loc)
            }
        }
    }
    for y in 0..dims.y {
        for x in 0..dims.x {
            let loc = Vec2i::new(x, y);
            if !res.contains(&loc) {
                print!("{}", map[&loc]);
            } else {
                print!("O");
            }
        }
        print!("\n");
    }

    println!("{:?}", res);
    res
}

fn greedy_compression(input: &Vec<String>) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let mut a: Vec<String>;
    let mut a_locs: Vec<usize>;
    let mut b: Vec<String>;
    let mut b_locs: Vec<usize>;
    let mut c: Vec<String>;
    let mut c_locs: Vec<usize>;
    let mut compressed: Vec<String> = Vec::new();
    for l in 4..10 {
        let mut remainders: VecDeque<Vec<String>> = VecDeque::new();
        let mut cp = input.to_vec();
        let mut iter = input.windows(l);
        let substr = iter.next().unwrap();
        a = substr.to_vec();

        a_locs = {
            let mut indices_to_sub: Vec<usize> = iter
                .enumerate()
                .filter(|(_, x)| *x == substr)
                .map(|(i, _)| i + 1)
                .collect::<Vec<_>>();
            indices_to_sub.insert(0, 0);
            indices_to_sub
        };

        for i in a_locs.iter().rev() {
            let rem = cp.split_off(*i);
            remainders.push_front(rem[l..].to_vec());
        }
        remainders.push_front(cp.to_vec());
        remainders.retain(|x| !x.is_empty());

        b = remainders.front().unwrap().to_vec();
        if b.len() > 15 {
            continue;
        }
        b_locs = input
            .windows(b.len())
            .enumerate()
            .filter(|(_, x)| (*x).to_vec() == b)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let mut remainders_after_b: VecDeque<Vec<String>> = VecDeque::new();
        for r in remainders.iter_mut().rev() {
            let mut indices_to_sub = r
                .windows(b.len())
                .enumerate()
                .filter(|(_, x)| (*x).to_vec() == b)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            for i in indices_to_sub.iter().rev() {
                let rem = r.split_off(*i);
                remainders_after_b.push_front(rem[b.len()..].to_vec());
            }
            remainders_after_b.push_front(r.to_vec());
        }
        remainders_after_b.retain(|x| !x.is_empty());
        remainders = remainders_after_b;

        c = remainders.front().unwrap().to_vec();
        eprintln!("A: {:?}", a);
        eprintln!("B: {:?}", b);
        eprintln!("C: {:?}", c);
        c_locs = input
            .windows(c.len())
            .enumerate()
            .filter(|(_, x)| (*x).to_vec() == c)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let mut remainders_after_c: VecDeque<Vec<String>> = VecDeque::new();
        for r in remainders.iter_mut().rev() {
            let mut indices_to_sub = r
                .windows(c.len())
                .enumerate()
                .filter(|(_, x)| (*x).to_vec() == c)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            for i in indices_to_sub.iter().rev() {
                let rem = r.split_off(*i);
                remainders_after_c.push_front(rem[c.len()..].to_vec());
            }
            remainders_after_c.push_front(r.to_vec());
        }
        remainders_after_c.retain(|x| !x.is_empty());
        remainders = remainders_after_c;
        if remainders.is_empty() {
            eprintln!("got it");
            eprintln!("A: {:?}", a);
            eprintln!("B: {:?}", b);
            eprintln!("C: {:?}", c);

            let mut loc_sub_pairs = a_locs
                .iter()
                .map(|x| (x, "A".to_string()))
                .chain(b_locs.iter().map(|x| (x, "B".to_string())))
                .chain(c_locs.iter().map(|x| (x, "C".to_string())))
                .collect::<Vec<_>>();
            loc_sub_pairs.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
            compressed = loc_sub_pairs
                .into_iter()
                .map(|(_, s)| s)
                .collect::<Vec<_>>();

            eprintln!("cpr: {:?}", compressed);
            return (a, b, c, compressed);
        }
    }
    panic!("aayyy");
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

    // load program
    let no_whitespace_str: String = file_string
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("");
    let program: Vec<i64> = no_whitespace_str
        .split(",")
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let map = run_game(program.to_vec());

    let w = map.keys().map(|v| v.x).max().unwrap() + 1;
    let h = map.len() as i32 / w;

    let inters = get_inters(Vec2i::new(w, h), &map);

    let sum = inters.iter().map(|l| l.x * l.y).sum::<i32>();
    println!("Sum: {}", sum);

    let (trajs, _) = get_vertices(Vec2i::new(w, h), &map);

    let min = trajs
        .iter()
        .min_by(|x, y| x.len().partial_cmp(&y.len()).unwrap())
        .unwrap();

    let moves = min
        .match_indices(|c| c == 'R' || c == 'L')
        .map(|(_, ss)| ss)
        .filter(|x| *x != "")
        .collect::<Vec<_>>();
    let steps = min
        .split(|c| c == 'R' || c == 'L')
        .filter(|x| *x != "")
        .collect::<Vec<_>>();
    let mut min_vec: Vec<String> = Vec::new();
    for (m, s) in moves.iter().zip(steps.iter()) {
        min_vec.push(m.to_string());
        min_vec.push(s.to_string());
    }
    let (a, b, c, comp) = greedy_compression(&min_vec);
    let mut new_program = program.to_vec();
    new_program[0] = 2;

    let mut prog_input: Vec<i64> = Vec::new();
    for abc in [comp, a, b, c].iter() {
        for aa in abc {
            for ch in aa.chars() {
                prog_input.push(ch as i64);
            }
            prog_input.push(',' as i64);
        }
        prog_input.pop();
        prog_input.push('\n' as i64);
    }
    prog_input.push('n' as i64);
    prog_input.push('\n' as i64);
    prog_input.push('\n' as i64);
    run_game_b(new_program, prog_input);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}

    #[test]
    fn test2() {}
}
