use std::cell::RefCell;
use std::collections::HashSet;
use std::io::Read;
use std::iter;
extern crate itertools;
use itertools::Itertools;

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

fn process<I, O>(prog: &mut Vec<i32>, mut pc: usize, input: &mut I, output: &mut O) -> usize
where
    I: FnMut() -> Option<String>,
    O: FnMut(i32) -> (),
{
    loop {
        let cached_pc = pc;
        let instr = parse_next_instr(&mut pc, &prog);

        match instr {
            ParsedInstruction::Add { op1, op2, dest } => {
                prog[dest] = op1 + op2;
            }
            ParsedInstruction::Multiply { op1, op2, dest } => {
                prog[dest] = op1 * op2;
            }
            ParsedInstruction::Input { dest } => match input() {
                Some(line) => {
                    eprintln!("Input {} -> {}", line, dest);
                    prog[dest] = line.parse::<i32>().unwrap();
                }
                None => {
                    eprintln!("Breaking to wait for input");
                    pc = cached_pc;
                    break;
                }
            },
            ParsedInstruction::Output { src } => {
                let out = prog[src];
                eprintln!("Output: {}", out);
                output(out);
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
                prog[dest] = if op1 < op2 { 1 } else { 0 };
            }

            ParsedInstruction::Equals { op1, op2, dest } => {
                prog[dest] = if op1 == op2 { 1 } else { 0 };
            }

            ParsedInstruction::Halt => {
                eprintln!("HALTING");
                pc = std::usize::MAX;
                break;
            }
        }
    }

    pc
}

fn test_sequence(prog: Vec<i32>, mut phase_sequence: Vec<i32>) -> i32 {
    let buffer: RefCell<Vec<i32>> = RefCell::new(vec![phase_sequence.remove(0), 0]);

    let mut buffer_input = || {
        if buffer.borrow().is_empty() {
            None
        } else {
            Some(buffer.borrow_mut().remove(0).to_string())
        }
    };
    let mut buffer_output = |x: i32| {
        if !phase_sequence.is_empty() {
            buffer.borrow_mut().push(phase_sequence.remove(0));
        }
        buffer.borrow_mut().push(x);
    };

    for _ in 0..5 {
        let mut fresh_prog = prog.to_vec();
        process(&mut fresh_prog, 0, &mut buffer_input, &mut buffer_output);
    }
    println!("{:?}", buffer.borrow());
    return *buffer.borrow().first().unwrap();
}

fn test_sequence_feeback(prog: Vec<i32>, phase_sequence: Vec<i32>) -> i32 {
    eprintln!("Testing seq: {:?}", phase_sequence);
    let mut phase_iter = phase_sequence.iter();
    let buffer_a: RefCell<Vec<i32>> = RefCell::new(vec![*phase_iter.next().unwrap(), 0]);
    let buffer_b: RefCell<Vec<i32>> = RefCell::new(vec![*phase_iter.next().unwrap()]);
    let buffer_c: RefCell<Vec<i32>> = RefCell::new(vec![*phase_iter.next().unwrap()]);
    let buffer_d: RefCell<Vec<i32>> = RefCell::new(vec![*phase_iter.next().unwrap()]);
    let buffer_e: RefCell<Vec<i32>> = RefCell::new(vec![*phase_iter.next().unwrap()]);

    let buffers: Vec<&RefCell<Vec<i32>>> =
        vec![&buffer_a, &buffer_b, &buffer_c, &buffer_d, &buffer_e];

    let mut program_bank: Vec<Vec<i32>> = iter::repeat(prog).take(5).collect();
    let mut prog_counters: Vec<usize> = vec![0, 0, 0, 0, 0];
    let mut finished_amps: HashSet<usize> = HashSet::new();
    let test_finished_amps: HashSet<usize> = [0, 1, 2, 3, 4].iter().cloned().collect();

    for i in (0..5).cycle() {
        let prog = program_bank.get_mut(i).unwrap();
        let pc = prog_counters.get_mut(i).unwrap();
        eprintln!("running amp {}", i);

        let mut buffer_input = || {
            let buffer = buffers[i];
            eprintln!("Buffer (in) Contents: {:?}", buffer.borrow());
            if buffer.borrow().is_empty() {
                None
            } else {
                Some(buffer.borrow_mut().remove(0).to_string())
            }
        };
        let mut buffer_output = |x: i32| {
            let buffer = buffers[(i + 1) % 5];
            buffer.borrow_mut().push(x);
            eprintln!("Buffer Contents: {:?}", buffer.borrow());
        };

        *pc = process(prog, *pc, &mut buffer_input, &mut buffer_output);
        if *pc == std::usize::MAX {
            finished_amps.insert(i);
            *pc = Opcode::Halt as usize;
        }
        if finished_amps == test_finished_amps {
            break;
        }
    }
    //eprintln!("{:?}", buffer.borrow());
    return *buffers[0].borrow().first().unwrap();
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

    // let max = (0..5)
    //     .permutations(5)
    //     .map(|x| test_sequence(program.to_vec(), x))
    //     .max()
    //     .unwrap();

    let max_feedback = (5..10)
        .permutations(5)
        .map(|x| test_sequence_feeback(program.to_vec(), x))
        .max()
        .unwrap();
    eprintln!("MaxFeedBack {}", max_feedback);

    // test_sequence(program.to_vec(), vec![0, 1, 2, 3, 4]);
    // let mut stdin_input = || {
    //     let stdin = io::stdin();
    //     let s: String = stdin.lock().lines().next().unwrap().unwrap();
    //     return s;
    // };
    // let mut stdout_output = |x: i32| {
    //     let mut stdout = io::stdout();
    //     let mut s = x.to_string();
    //     s.push('\n');
    //     stdout
    //         .write(s.to_string().as_bytes())
    //         .expect("write failed");
    // };
    // process(program.to_vec(), &mut stdin_input, &mut stdout_output);
    // process(program.to_vec(), &mut stdin_input, &mut stdout_output);
    // process(program.to_vec(), &mut stdin_input, &mut stdout_output);
}

#[cfg(test)]
mod tests {

    use test_sequence_feeback;

    #[test]
    fn test1() {
        let test_prog: Vec<i32> = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phase_seq: Vec<i32> = vec![9, 8, 7, 6, 5];
        let res = test_sequence_feeback(test_prog.to_vec(), phase_seq.to_vec());
        assert_eq!(res, 139629729);
    }

    #[test]
    fn test2() {
        let test_prog: Vec<i32> = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phase_seq: Vec<i32> = vec![9, 7, 8, 5, 6];
        let res = test_sequence_feeback(test_prog.to_vec(), phase_seq.to_vec());
        assert_eq!(res, 18216);
    }
}
