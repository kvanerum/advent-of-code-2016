use regex::{Match, Regex};
use std::collections::HashMap;
use std::fs;

struct Instruction {
    operation: String,
    param1: Parameter,
    param2: Option<Parameter>,
}

enum Parameter {
    Register(char),
    Number(i32),
}

fn main() {
    let parser = Regex::new(
        r"^(?<op>[a-z]+) (?:(?<p1r>[a-d])|(?<p1n>-?\d+))(?: (?:(?<p2r>[a-d])|(?<p2n>-?\d+)))?$",
    )
    .expect("parse regex");
    let input: Vec<Instruction> = fs::read_to_string("day25/resources/input.txt")
        .expect("read input file")
        .lines()
        .map(|s| parse_instruction(s, &parser))
        .collect();

    let mut a: i32 = 0;

    while !run(&input, a) {
        a += 1;
    }

    println!("a: {}", a);
}

fn run(instructions: &Vec<Instruction>, init_a: i32) -> bool {
    let mut instruction_pointer = 0_usize;
    let mut registers: HashMap<char, i32> =
        HashMap::from([('a', init_a), ('b', 0), ('c', 0), ('d', 0)]);
    let mut last_output: Option<i32> = None;
    let mut output_count: usize = 0;

    while instruction_pointer < instructions.len() {
        let instruction = instructions.get(instruction_pointer).expect("instruction");

        if instruction.operation == "out" {
            let value = match instruction.param1 {
                Parameter::Register(register) => *registers.get(&register).expect("valid register"),
                Parameter::Number(number) => number,
            };

            if last_output.is_none() || last_output.unwrap() != value {
                output_count += 1;
                last_output = Some(value);

                if output_count == 100 {
                    return true;
                }
            } else {
                return false;
            }
        } else {
            match instruction.operation.as_str() {
                "cpy" => cpy(instruction, &mut registers),
                "inc" => inc(instruction, &mut registers),
                "dec" => dec(instruction, &mut registers),
                "jnz" => jnz(instruction, &mut registers, &mut instruction_pointer),
                _ => panic!("invalid instruction {}", instruction.operation),
            }
        }

        instruction_pointer += 1;
    }

    false
}

fn cpy(instruction: &Instruction, registers: &mut HashMap<char, i32>) {
    let value = match instruction.param1 {
        Parameter::Register(register) => *registers.get(&register).expect("valid register"),
        Parameter::Number(number) => number,
    };

    if let Some(Parameter::Register(target)) = instruction.param2 {
        registers.insert(target, value);
    } else {
        panic!("invalid instruction")
    }
}

fn inc(instruction: &Instruction, registers: &mut HashMap<char, i32>) {
    if let Parameter::Register(target) = instruction.param1 {
        *registers.get_mut(&target).expect("register") += 1;
    } else {
        panic!("invalid instruction")
    }
}

fn dec(instruction: &Instruction, registers: &mut HashMap<char, i32>) {
    if let Parameter::Register(target) = instruction.param1 {
        *registers.get_mut(&target).expect("register") -= 1;
    } else {
        panic!("invalid instruction")
    }
}

fn jnz(
    instruction: &Instruction,
    registers: &mut HashMap<char, i32>,
    instruction_pointer: &mut usize,
) {
    let value = match instruction.param1 {
        Parameter::Register(register) => *registers.get(&register).expect("valid register"),
        Parameter::Number(number) => number,
    };

    if value == 0 {
        return;
    }

    if let Some(Parameter::Number(offset)) = instruction.param2 {
        let o = offset - 1;

        if o.is_negative() {
            *instruction_pointer -= o.abs() as usize;
        } else {
            *instruction_pointer += o as usize;
        }
    } else {
        panic!("invalid instruction")
    }
}

fn parse_instruction(instruction: &str, regex: &Regex) -> Instruction {
    let capture = regex.captures(instruction).expect("parse line");
    let operation = capture.name("op").expect("op").as_str();
    let p1r = capture.name("p1r");
    let p1n = capture.name("p1n");
    let p2r = capture.name("p2r");
    let p2n = capture.name("p2n");

    let param1: Parameter = parse_parameter_number(p1n)
        .or(parse_parameter_register(p1r))
        .expect("Parameter");

    let param2 = parse_parameter_number(p2n).or(parse_parameter_register(p2r));

    return Instruction {
        operation: operation.to_string(),
        param1: param1,
        param2: param2,
    };
}

fn parse_parameter_number(parameter: Option<Match>) -> Option<Parameter> {
    return parameter
        .map(|n| n.as_str().parse().expect("number"))
        .map(|n| Parameter::Number(n));
}

fn parse_parameter_register(parameter: Option<Match>) -> Option<Parameter> {
    return parameter
        .map(|r| r.as_str().chars().nth(0).expect("parameter"))
        .map(|r| Parameter::Register(r));
}
