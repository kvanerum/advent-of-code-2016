use std::fs;

fn main() {
    let example = "day20/resources/example.txt";
    let input = "day20/resources/input.txt";

    let mut blacklisted_ip_ranges = read_input(input);
    blacklisted_ip_ranges.sort_by_key(|ip_range| ip_range.0);

    let mut allowed_ip_ranges = Vec::new();
    let mut current_ip = 0;

    for blacklisted_ip_range in blacklisted_ip_ranges {
        if blacklisted_ip_range.0 <= current_ip && current_ip <= blacklisted_ip_range.1 {
            current_ip = blacklisted_ip_range.1 + 1;
        } else if blacklisted_ip_range.0 > current_ip {
            allowed_ip_ranges.push((current_ip, blacklisted_ip_range.0 - 1));
            current_ip = blacklisted_ip_range.1;

            if current_ip < u32::MAX {
                current_ip += 1;
            }
        }
    }

    if current_ip < u32::MAX {
        allowed_ip_ranges.push((current_ip, u32::MAX));
    }

    let part_1 = allowed_ip_ranges.get(0).map(|range| range.0).unwrap();
    println!("{}", part_1);

    let part_2 = allowed_ip_ranges
        .iter()
        .map(|range| range.1 - range.0 + 1)
        .sum::<u32>();
    println!("{}", part_2);
}

fn read_input(filename: &str) -> Vec<(u32, u32)> {
    fs::read_to_string(filename)
        .expect("read input file")
        .trim_end()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}
