extern crate rustbox;
use rustbox::{InitOptions, RustBox};
use std::cell::RefCell;
use std::io::{stdin, stdout, BufRead, Read, Write};
extern crate nalgebra as na;
type Vec2i = na::Vector2<i32>;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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
    Open = 0,
    Wall = 1,
    Sensor = 2,
}

impl Tile {
    fn from_i64(value: i64) -> Result<Tile, String> {
        match value {
            1 => Ok(Tile::Open),
            0 => Ok(Tile::Wall),
            2 => Ok(Tile::Sensor),
            _ => Err(format!("Invalid Tile: {}", value)),
        }
    }
}

fn run_game(mut prog: Vec<i64>) -> HashMap<Vec2i, Tile> {
    #[derive(Debug)]
    struct Trav {
        pos: Vec2i,
        trajectory: VecDeque<i64>,
    };
    //let mut trav_queue: VecDeque<Trav> = VecDeque::new();
    //#[derive(Debug)]
    struct ProgState {
        current_pos: Vec2i,
        current_trav: Trav,
        map: HashMap<Vec2i, Tile>,
        delta_pos: Vec2i,
        trav_queue: VecDeque<Trav>,
        command_queue: VecDeque<i64>,
        rb: RustBox,
    };
    let prog_state_ref: RefCell<ProgState> = RefCell::new(ProgState {
        current_pos: Vec2i::new(0, 0),
        current_trav: Trav {
            pos: Vec2i::new(0, 0),
            trajectory: VecDeque::new(),
        },
        map: HashMap::new(),
        delta_pos: Vec2i::new(0, 0),
        trav_queue: VecDeque::new(),
        command_queue: VecDeque::new(),
        rb: RustBox::init(InitOptions {
            input_mode: rustbox::InputMode::Esc,
            ..Default::default()
        })
        .unwrap(),
    });
    {
        let mut p = prog_state_ref.borrow_mut();
        p.map.insert(Vec2i::new(0, 0), Tile::Open);
        p.trav_queue.push_back(Trav {
            pos: Vec2i::new(0, 1),
            trajectory: vec![1].into_iter().collect(),
        });
        p.trav_queue.push_back(Trav {
            pos: Vec2i::new(0, -1),
            trajectory: vec![2].into_iter().collect(),
        });
        p.trav_queue.push_back(Trav {
            pos: Vec2i::new(-1, 0),
            trajectory: vec![3].into_iter().collect(),
        });
        p.trav_queue.push_back(Trav {
            pos: Vec2i::new(1, 0),
            trajectory: vec![4].into_iter().collect(),
        });
    }

    let mut input = || {
        let mut p = prog_state_ref.borrow_mut();

        if p.current_pos == p.current_trav.pos {
            // get the state of the last command, if we're here,
            // there has to be something in the map, that's a precondition
            let &cur_tile = p.map.get(&p.current_pos).unwrap();
            if cur_tile == Tile::Sensor {
                p.rb.print(
                    15 as usize,
                    50 as usize,
                    rustbox::RB_NORMAL,
                    rustbox::Color::White,
                    rustbox::Color::Black,
                    &("found it: ".to_string() + &p.current_trav.trajectory.len().to_string()),
                );
                p.rb.present();
            }

            let neighs: [(i64, Vec2i); 4] = [
                (1, Vec2i::new(0, 1)),  // north
                (2, Vec2i::new(0, -1)), // south
                (3, Vec2i::new(-1, 0)), // west
                (4, Vec2i::new(1, 0)),  // east
            ];

            for (c, d) in neighs.iter() {
                let new_pos = d + p.current_pos;
                if !p.map.contains_key(&new_pos) {
                    let mut new_traj = p.current_trav.trajectory.clone();
                    new_traj.push_back(*c);
                    p.trav_queue.push_back(Trav {
                        pos: new_pos,
                        trajectory: new_traj,
                    });
                }
            }
        }
        if p.command_queue.is_empty() {
            if p.trav_queue.is_empty() {
                return Some(0.to_string());
            }
            p.current_trav = p.trav_queue.pop_front().unwrap();
            // we need to go here
            let mut new_traj: VecDeque<i64> = p.current_trav.trajectory.clone();

            let mut rev_traj = new_traj
                .iter()
                .rev()
                .map(|x| match x {
                    1 => 2,
                    2 => 1,
                    3 => 4,
                    4 => 3,
                    _ => panic!(""),
                })
                .collect();
            p.command_queue.append(&mut new_traj);
            p.command_queue.append(&mut rev_traj);
        }

        let c = p.command_queue.pop_front().unwrap();
        p.delta_pos = match c {
            1 => Vec2i::new(0, 1),
            2 => Vec2i::new(0, -1),
            3 => Vec2i::new(-1, 0),
            4 => Vec2i::new(1, 0),
            _ => panic!(""),
        };
        Some(c.to_string())
    };

    //out
    let mut output = |x: i64| {
        let tile = Tile::from_i64(x).unwrap();
        let mut p = prog_state_ref.borrow_mut();
        let (pos, c) = match tile {
            Tile::Open | Tile::Sensor => {
                p.current_pos += p.delta_pos;
                let cp = p.current_pos;
                p.map.entry(cp).or_insert(tile);
                (cp, '.')
            }
            Tile::Wall => {
                let cp = p.current_pos + p.delta_pos;
                p.delta_pos = Vec2i::new(0, 0);
                p.command_queue.pop_front();
                p.map.entry(cp).or_insert(tile);
                (cp, '□')
            }
        };
        let origin = Vec2i::new(-40, -20);
        let offset = pos - origin;
        p.rb.print_char(
            offset.x as usize,
            offset.y as usize,
            rustbox::RB_NORMAL,
            rustbox::Color::White,
            rustbox::Color::Black,
            c,
        );
        p.rb.present();
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

    let mut bfs: VecDeque<(Vec2i, i32)> = VecDeque::new();
    let mut sensor = Vec2i::new(0, 0);
    for (&k, &v) in &map {
        if v == Tile::Sensor {
            sensor = k;
        }
    }
    bfs.push_back((sensor, 0));
    let mut visited: HashSet<Vec2i> = HashSet::new();

    let mut max: i32 = 0;
    while !bfs.is_empty() {
        let cur = bfs.pop_front().unwrap();

        if visited.contains(&cur.0) {
            continue;
        }
        visited.insert(cur.0);

        if cur.1 > max {
            max = cur.1;
            eprintln!("new max: {}", max);
        }

        let neighs: [Vec2i; 4] = [
            Vec2i::new(0, 1),  // north
            Vec2i::new(0, -1), // south
            Vec2i::new(-1, 0), // west
            Vec2i::new(1, 0),  // east
        ];

        for d in neighs.iter() {
            let new_pos = d + cur.0;
            if map.contains_key(&new_pos) {
                if *(map.get(&new_pos).unwrap()) != Tile::Wall {
                    bfs.push_back((new_pos, cur.1 + 1));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}

    #[test]
    fn test2() {}
}
