use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("day9/resources/input.txt").expect("read input file");

    println!("{}", decompress_size(&input.trim(), false));
    println!("{}", decompress_size(&input.trim(), true));
}

fn decompress_size(input: &str, decompress_data: bool) -> usize {
    let mut index: usize = 0;
    let mut size: usize = 0;

    while index < input.len() {
        let current_char = input.chars().nth(index).expect("char");

        if current_char == '(' {
            let marker_length = input
                .chars()
                .skip(index)
                .position(|c| c == ')')
                .expect("closing marker");

            let marker: String = input
                .chars()
                .skip(index + 1)
                .take(marker_length - 1)
                .collect();

            let parsed_marker = parse_marker(&marker);

            index = index + marker_length + 1;

            if decompress_data {
                let data: String = input.chars().skip(index).take(parsed_marker.0).collect();
                let data_total_size = decompress_size(&data, true);
                size += data_total_size * parsed_marker.1;
            } else {
                size += parsed_marker.0 * parsed_marker.1;
            }

            index += parsed_marker.0;
        } else {
            size += 1;
            index += 1;
        }
    }

    return size;
}

fn parse_marker(marker: &String) -> (usize, usize) {
    return marker
        .split("x")
        .map(|x| x.parse::<usize>().expect("numerical"))
        .next_tuple()
        .expect("valid marker");
}
