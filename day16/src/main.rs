struct Input<'a> {
    initial_state: &'a str,
    target_length: usize,
}

fn main() {
    let example = Input {
        initial_state: "10000",
        target_length: 20,
    };

    let input = example;

    let mut state = input_to_bool_vec(input.initial_state);

    while state.len() < input.target_length {
        extend(&mut state);
    }

    state.truncate(input.target_length);

    let mut checksum = calculate_checksum(&state);

    while checksum.len() % 2 == 0 {
        checksum = calculate_checksum(&checksum);
    }

    println!("{}", bool_vec_to_string(&checksum));
}

fn extend(state: &mut Vec<bool>) -> &mut Vec<bool> {
    let original_length = state.len();
    state.push(false);

    for i in (0..original_length).rev() {
        state.push(!state[i]);
    }

    state
}

fn calculate_checksum(input: &Vec<bool>) -> Vec<bool> {
    let mut result = Vec::new();

    let mut i = 0;

    while i < input.len() {
        if input[i] == input[i + 1] {
            result.push(true);
        } else {
            result.push(false);
        }

        i += 2;
    }

    result
}

fn input_to_bool_vec(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '1').collect()
}

fn bool_vec_to_string(input: &Vec<bool>) -> String {
    input.iter().map(|b| if *b { '1' } else { '0' }).collect()
}
