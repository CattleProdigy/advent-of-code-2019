use std::io::{self, BufRead, Read};

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
    Halt = 99,
}

impl Opcode {
    fn from_i32(value: i32) -> Result<Opcode, String> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpTrue),
            6 => Ok(Opcode::JumpFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            99 => Ok(Opcode::Halt),
            _ => Err(format!("Invalid Opcode: {}", value)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum OperandMode {
    POSITION = 0,
    IMMEDIATE = 1,
}

impl OperandMode {
    fn from_i32(value: i32) -> Result<OperandMode, String> {
        match value {
            0 => Ok(OperandMode::POSITION),
            1 => Ok(OperandMode::IMMEDIATE),
            _ => Err(format!("Invalid OperandMode: {}", value)),
        }
    }
}

fn operand_modes(x: i32) -> [OperandMode; 3] {
    let mut res: [OperandMode; 3] = [OperandMode::POSITION; 3];

    let mut x_div = x;
    res[0] = OperandMode::from_i32(x_div % 10).expect("");
    x_div = x_div / 10;
    res[1] = OperandMode::from_i32(x_div % 10).expect("");
    x_div = x_div / 10;
    res[2] = OperandMode::from_i32(x_div % 10).expect("");

    res
}

enum ParsedInstruction {
    Add { op1: i32, op2: i32, dest: usize },
    Multiply { op1: i32, op2: i32, dest: usize },
    Input { dest: usize },
    Output { src: usize },
    JumpTrue { test: i32, jump_dest: usize },
    JumpFalse { test: i32, jump_dest: usize },
    LessThan { op1: i32, op2: i32, dest: usize },
    Equals { op1: i32, op2: i32, dest: usize },
    Halt,
}

fn load_operands<'a>(
    op: Opcode,
    op_modes: [OperandMode; 3],
    iter: &mut usize,
    prog: &Vec<i32>,
) -> ParsedInstruction {
    let mut parse_operand = |i| -> i32 {
        let op = if i == OperandMode::POSITION {
            prog[prog[*iter] as usize]
        } else {
            prog[*iter]
        };
        *iter += 1;
        op
    };

    match op {
        Opcode::Add => ParsedInstruction::Add {
            op1: parse_operand(op_modes[0]),
            op2: parse_operand(op_modes[1]),
            dest: parse_operand(OperandMode::IMMEDIATE) as usize,
        },
        Opcode::Multiply => ParsedInstruction::Multiply {
            op1: parse_operand(op_modes[0]),
            op2: parse_operand(op_modes[1]),
            dest: parse_operand(OperandMode::IMMEDIATE) as usize,
        },
        Opcode::Input => ParsedInstruction::Input {
            dest: parse_operand(OperandMode::IMMEDIATE) as usize,
        },
        Opcode::Output => ParsedInstruction::Output {
            src: parse_operand(OperandMode::IMMEDIATE) as usize,
        },
        Opcode::JumpTrue => ParsedInstruction::JumpTrue {
            test: parse_operand(op_modes[0]),
            jump_dest: parse_operand(op_modes[1]) as usize,
        },
        Opcode::JumpFalse => ParsedInstruction::JumpFalse {
            test: parse_operand(op_modes[0]),
            jump_dest: parse_operand(op_modes[1]) as usize,
        },
        Opcode::LessThan => ParsedInstruction::LessThan {
            op1: parse_operand(op_modes[0]),
            op2: parse_operand(op_modes[1]),
            dest: parse_operand(OperandMode::IMMEDIATE) as usize,
        },
        Opcode::Equals => ParsedInstruction::Equals {
            op1: parse_operand(op_modes[0]),
            op2: parse_operand(op_modes[1]),
            dest: parse_operand(OperandMode::IMMEDIATE) as usize,
        },
        Opcode::Halt => ParsedInstruction::Halt,
    }
}

fn parse_next_instr<'a>(iter: &mut usize, prog: &Vec<i32>) -> ParsedInstruction {
    let combined_opcode = prog[*iter];
    *iter += 1;
    let opcode_int = combined_opcode % 100;
    let operand_modes_int = combined_opcode / 100;
    let opcode = Opcode::from_i32(opcode_int).unwrap();
    let operand_modes = operand_modes(operand_modes_int);

    load_operands(opcode, operand_modes, iter, prog)
}

fn process(mut input_prog: Vec<i32>) {
    let mut pc: usize = 0;
    loop {
        let instr = parse_next_instr(&mut pc, &input_prog);

        match instr {
            ParsedInstruction::Add { op1, op2, dest } => {
                input_prog[dest] = op1 + op2;
            }
            ParsedInstruction::Multiply { op1, op2, dest } => {
                input_prog[dest] = op1 * op2;
            }
            ParsedInstruction::Input { dest } => {
                println!("Input {}", dest);
                let stdin = io::stdin();
                let line1 = stdin.lock().lines().next().unwrap().unwrap();
                input_prog[dest] = line1.parse::<i32>().unwrap();
            }
            ParsedInstruction::Output { src } => {
                println!("Output");
                let out = input_prog[src];
                println!("{}", out);
            }
            ParsedInstruction::JumpTrue { test, jump_dest } => {
                if test != 0 {
                    pc = jump_dest;
                }
            }
            ParsedInstruction::JumpFalse { test, jump_dest } => {
                if test == 0 {
                    pc = jump_dest;
                }
            }
            ParsedInstruction::LessThan { op1, op2, dest } => {
                input_prog[dest] = if op1 < op2 { 1 } else { 0 };
            }

            ParsedInstruction::Equals { op1, op2, dest } => {
                input_prog[dest] = if op1 == op2 { 1 } else { 0 };
            }

            ParsedInstruction::Halt => {
                println!("HALTING");
                break;
            }
        }
    }
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
    let program: Vec<i32> = no_whitespace_str
        .split(",")
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    process(program);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}
}
