use std::io::prelude::*;

fn process(input_prog: &Vec<usize>) -> Vec<usize> {
    let mut prog = input_prog.to_vec();
    let mut prog_counter: usize = 0;

    const ADD: usize = 1;
    const MULTIPLY: usize = 2;
    const HALT: usize = 99;

    while {
        let opcode = prog[prog_counter];
        match opcode {
            ADD => {
                let operand1 = prog[prog_counter + 1];
                let operand2 = prog[prog_counter + 2];
                let dest = prog[prog_counter + 3];
                prog[dest] = prog[operand1] + prog[operand2];
            }
            MULTIPLY => {
                let operand1 = prog[prog_counter + 1];
                let operand2 = prog[prog_counter + 2];
                let dest = prog[prog_counter + 3];
                prog[dest] = prog[operand1] * prog[operand2];
            }
            HALT => (), // do nothing, we'll break later
            _ => panic!("unexpected opcode"),
        }

        prog_counter += 4;

        opcode != HALT
    } {}

    prog.to_vec()
}

fn find_noun_verb(input_prog: &Vec<usize>) -> (usize, usize) {
    let mut noun_verb = (0, 0);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut patched = input_prog.to_vec();
            patched[1] = noun;
            patched[2] = verb;
            let result = process(&patched);
            if result[0] == 19690720 {
                noun_verb = (noun, verb);
            }
        }
    }

    noun_verb
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
    let int_strs: Vec<&str> = no_whitespace_str.split(",").collect();
    let program: Vec<usize> = int_strs
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // patch program
    let mut patched_program = program.to_vec();
    patched_program[1] = 12;
    patched_program[2] = 2;

    println!("before program run {:?}", patched_program);
    let result = process(&patched_program);
    println!("after program run {:?}", result);

    let noun_verb = find_noun_verb(&program);
    let noun_verb_result = noun_verb.0 * 100 + noun_verb.1;
    println!("Noun: {}, Verb: {}, 100xN+V: {}", noun_verb.0, noun_verb.1, noun_verb_result);
}

#[cfg(test)]
mod tests {
    use process;

    #[test]
    fn test1() {
        let test_input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let test_result = process(&test_input);
        let test_answer = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(test_result, test_answer);
    }
    #[test]
    fn test2() {
        let test_input = vec![1, 0, 0, 0, 99];
        let test_result = process(&test_input);
        let test_answer = vec![2, 0, 0, 0, 99];
        assert_eq!(test_result, test_answer);
    }
    #[test]
    fn test3() {
        let test_input = vec![2, 3, 0, 3, 99];
        let test_result = process(&test_input);
        let test_answer = vec![2, 3, 0, 6, 99];
        assert_eq!(test_result, test_answer);
    }
    #[test]
    fn test4() {
        let test_input = vec![2, 4, 4, 5, 99, 0];
        let test_result = process(&test_input);
        let test_answer = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(test_result, test_answer);
    }
    #[test]
    fn test5() {
        let test_input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let test_result = process(&test_input);
        let test_answer = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(test_result, test_answer);
    }

}
