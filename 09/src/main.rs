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
                OperandMode::Position =>  raw_opand_val,
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
        eprintln!("{:?}: {:?}", cached_pc, instr);

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
        let prog: Vec<i64> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let prog3: Vec<i64> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let prog2: Vec<i64> = vec![104, 1125899906842624, 99];
        run_with_stdin_stdout(prog.to_vec());
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

    run_with_stdin_stdout(program.to_vec());
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}

    #[test]
    fn test2() {}
}
