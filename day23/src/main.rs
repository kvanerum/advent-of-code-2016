use regex::{Match, Regex};
use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Instruction {
    operation: String,
    param1: Parameter,
    param2: Option<Parameter>,
}

#[derive(Clone)]
enum Parameter {
    Register(char),
    Number(i32),
}

fn main() {
    let parser = Regex::new(
        r"^(?<op>[a-z]+) (?:(?<p1r>[a-d])|(?<p1n>-?\d+))(?: (?:(?<p2r>[a-d])|(?<p2n>-?\d+)))?$",
    )
    .expect("parse regex");
    let input: Vec<Instruction> = fs::read_to_string("day23/resources/input.txt")
        .expect("read input file")
        .lines()
        .map(|s| parse_instruction(s, &parser))
        .collect();

    run(&input, 7);
    run(&input, 12);
}

fn run(original_instructions: &Vec<Instruction>, register_a: i32) {
    let mut instruction_pointer = 0_usize;
    let mut registers: HashMap<char, i32> =
        HashMap::from([('a', register_a), ('b', 0), ('c', 0), ('d', 0)]);
    let mut instructions = original_instructions.clone();

    while instruction_pointer < instructions.len() {
        let instruction = instructions
            .get(instruction_pointer)
            .expect("instruction")
            .clone();
        match instruction.operation.as_str() {
            "cpy" => cpy(&instruction, &mut registers),
            "inc" => inc(&instruction, &mut registers),
            "dec" => dec(&instruction, &mut registers),
            "jnz" => jnz(&instruction, &mut registers, &mut instruction_pointer),
            "tgl" => tgl(
                &instruction,
                &mut registers,
                &mut instruction_pointer,
                &mut instructions,
            ),
            _ => panic!("invalid instruction {}", instruction.operation),
        }

        instruction_pointer += 1;
    }

    println!("register a={}", registers.get(&'a').expect("register a"));
}

fn tgl(
    instruction: &Instruction,
    registers: &mut HashMap<char, i32>,
    instruction_pointer: &mut usize,
    instructions: &mut Vec<Instruction>,
) {
    let value = match instruction.param1 {
        Parameter::Register(register) => *registers.get(&register).expect("valid register"),
        Parameter::Number(number) => number,
    };
    let instruction_location = *instruction_pointer + value as usize;

    if let Some(original_instruction) = instructions.get(instruction_location) {
        let new_instruction = match original_instruction.operation.as_str() {
            "inc" => Instruction {
                operation: "dec".to_string(),
                param1: original_instruction.param1.clone(),
                param2: original_instruction.param2.clone(),
            },
            "dec" | "tgl" => Instruction {
                operation: "inc".to_string(),
                param1: original_instruction.param1.clone(),
                param2: original_instruction.param2.clone(),
            },
            "cpy" => Instruction {
                operation: "jnz".to_string(),
                param1: original_instruction.param1.clone(),
                param2: original_instruction.param2.clone(),
            },
            "jnz" => Instruction {
                operation: "cpy".to_string(),
                param1: original_instruction.param1.clone(),
                param2: original_instruction.param2.clone(),
            },
            _ => panic!("invalid instruction {}", original_instruction.operation),
        };

        instructions[instruction_location] = new_instruction;
    }
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

    let offset = if let Some(Parameter::Number(offset)) = instruction.param2 {
        offset - 1
    } else if let Some(Parameter::Register(offset)) = instruction.param2 {
        *registers.get(&offset).expect("valid register") - 1
    } else {
        panic!("invalid instruction")
    };

    if offset.is_negative() {
        *instruction_pointer -= offset.abs() as usize;
    } else {
        *instruction_pointer += offset as usize;
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

    Instruction {
        operation: operation.to_string(),
        param1: param1,
        param2: param2,
    }
}

fn parse_parameter_number(parameter: Option<Match>) -> Option<Parameter> {
    parameter
        .map(|n| n.as_str().parse().expect("number"))
        .map(|n| Parameter::Number(n))
}

fn parse_parameter_register(parameter: Option<Match>) -> Option<Parameter> {
    parameter
        .map(|r| r.as_str().chars().nth(0).expect("parameter"))
        .map(|r| Parameter::Register(r))
}
