use std::fs;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

struct Instruction {
    method: String,
    param1: usize,
    param2: usize,
}

fn main() {
    let input: Vec<Instruction> = fs::read_to_string("day8/resources/input.txt")
        .expect("read input file")
        .lines()
        .map(|s| parse_instruction(s))
        .collect();

    let mut screen = vec![vec![false; WIDTH]; HEIGHT];

    input.iter().for_each(|instruction| {
        execute_instruction(instruction, &mut screen);
    });

    let count = screen
        .iter()
        .map(|row| row.iter().filter(|pixel| **pixel).count())
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{count}");

    render_screen(&screen);
}

fn render_screen(screen: &Vec<Vec<bool>>) {
    screen.iter().for_each(|line| {
        line.iter()
            .for_each(|pixel| print!("{}", if *pixel { "#" } else { "." }));
        println!();
    })
}

fn execute_instruction(instruction: &Instruction, screen: &mut Vec<Vec<bool>>) {
    match instruction.method.as_str() {
        "rect" => execute_instruction_rect(instruction, screen),
        "rotate row" => execute_instruction_rotate_row(instruction, screen),
        "rotate column" => execute_instruction_rotate_column(instruction, screen),
        _ => {
            panic!("invalid instruction {}", instruction.method)
        }
    }
}

fn execute_instruction_rect(instruction: &Instruction, screen: &mut Vec<Vec<bool>>) {
    for y in 0..instruction.param2 {
        for x in 0..instruction.param1 {
            screen[y][x] = true;
        }
    }
}

fn execute_instruction_rotate_row(instruction: &Instruction, screen: &mut Vec<Vec<bool>>) {
    let snapshot = screen[instruction.param1].clone();

    for i in 0..WIDTH {
        let new_position = (i + instruction.param2) % WIDTH;
        screen[instruction.param1][new_position] = snapshot[i];
    }
}

fn execute_instruction_rotate_column(instruction: &Instruction, screen: &mut Vec<Vec<bool>>) {
    let snapshot: Vec<bool> = (0..HEIGHT).map(|y| screen[y][instruction.param1]).collect();

    for i in 0..HEIGHT {
        let new_position = (i + instruction.param2) % HEIGHT;

        screen[new_position][instruction.param1] = snapshot[i];
    }
}

fn parse_instruction(input: &str) -> Instruction {
    if input.starts_with("rect ") {
        return parse_rect_instruction(input);
    } else if input.starts_with("rotate row y=") {
        return parse_rotate_instruction(input, "rotate row");
    } else if input.starts_with("rotate column x=") {
        return parse_rotate_instruction(input, "rotate column");
    } else {
        panic!("invalid instruction");
    }
}

fn parse_rect_instruction(input: &str) -> Instruction {
    let params: Vec<&str> = input[5..].split("x").collect();

    return Instruction {
        method: "rect".to_string(),
        param1: params
            .get(0)
            .expect("invalid instruction 1")
            .parse()
            .expect("invalid instruction 2"),
        param2: params
            .get(1)
            .expect("invalid instruction 3")
            .parse()
            .expect("invalid instruction 4"),
    };
}

fn parse_rotate_instruction(input: &str, instruction: &str) -> Instruction {
    let eq_position = input.find("=").expect("invalid instruction");
    let params: Vec<&str> = input[eq_position + 1..].split(" by ").collect();

    return Instruction {
        method: instruction.to_string(),
        param1: params
            .get(0)
            .expect("invalid instruction")
            .parse()
            .expect("invalid instruction"),
        param2: params
            .get(1)
            .expect("invalid instruction")
            .parse()
            .expect("invalid instruction"),
    };
}
