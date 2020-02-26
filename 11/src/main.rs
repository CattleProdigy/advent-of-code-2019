use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufRead, Read};

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
                    eprintln!("Input {} -> {}", line, dest);
                    *get_mut_ext(prog, dest) = line.parse::<i64>().unwrap();
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
                eprintln!("HALTING");
                pc.halted = true;
                break;
            }
        }
    }

    pc
}
fn run_robot(mut prog: Vec<i64>) {
    struct RobotState {
        dir: (i32, i32),
        loc: (i32, i32),
        painted_panels: HashMap<(i32, i32), i32>,
        needs_color: bool,
    };
    let robot_state_ref: RefCell<RobotState> = RefCell::new(RobotState {
        dir: (0, 1),
        loc: (0, 0),
        painted_panels: HashMap::new(),
        needs_color: true,
    });

    // For part 2
    {
        robot_state_ref
            .borrow_mut()
            .painted_panels
            .insert((0, 0), 1);
    }

    let mut buffer_input = || {
        let rs_borrowed = robot_state_ref.borrow();
        if rs_borrowed.painted_panels.contains_key(&rs_borrowed.loc) {
            Some(
                rs_borrowed
                    .painted_panels
                    .get(&rs_borrowed.loc)
                    .unwrap()
                    .to_string(),
            )
        } else {
            Some(0.to_string())
        }
    };
    let mut buffer_output = |x: i64| {
        let mut rs_borrowed = robot_state_ref.borrow_mut();
        if rs_borrowed.needs_color {
            let loc = rs_borrowed.loc;
            rs_borrowed.painted_panels.insert(loc, x as i32);
        } else {
            let dir_adj = if x == 1 { (0, 1, -1, 0) } else { (0, -1, 1, 0) };
            rs_borrowed.dir = (
                dir_adj.0 * rs_borrowed.dir.0 + dir_adj.1 * rs_borrowed.dir.1,
                dir_adj.2 * rs_borrowed.dir.0 + dir_adj.3 * rs_borrowed.dir.1,
            );
            rs_borrowed.loc = (
                rs_borrowed.loc.0 + rs_borrowed.dir.0,
                rs_borrowed.loc.1 + rs_borrowed.dir.1,
            );
        }
        rs_borrowed.needs_color = !rs_borrowed.needs_color;
    };
    let mut ps = ProcState {
        prog_count: 0,
        rel_base: 0,
        halted: false,
    };
    while !ps.halted {
        ps = process(&mut prog, ps, &mut buffer_input, &mut buffer_output);
    }
    println!(
        "Painted Panels: {}",
        robot_state_ref.borrow().painted_panels.len()
    );

    // Draw the panels
    {
        let rsb = robot_state_ref.borrow();
        let xs: Vec<i32> = rsb.painted_panels.iter().map(|(k, _)| k.0).collect();
        let ys: Vec<i32> = rsb.painted_panels.iter().map(|(k, _)| -1*k.1).collect();
        let maxx = xs.iter().max().unwrap();
        let minx = xs.iter().min().unwrap();
        let maxy = ys.iter().max().unwrap();
        let miny = ys.iter().min().unwrap();
        let width = maxx - minx + 1;
        let height = maxy - miny + 1;
        let mut img: Vec<char> = vec![' '; (width * height) as usize];
        for ((x, y), v) in &rsb.painted_panels {
            if *v == 1 {
                let adjusted_x = x - minx;
                let adjusted_y = -1*y - miny;
                let idx = (adjusted_x + width * adjusted_y) as usize;
                img[idx] = '█';
            }
        }
        for y in 0..height {
            for x in 0..width {
                let idx = (x + width * y) as usize;
                print!("{}", img[idx]);
            }
            print!("{}", '\n');
        }
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

    run_robot(program.to_vec());
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}

    #[test]
    fn test2() {}
}
