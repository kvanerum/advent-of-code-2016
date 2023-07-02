use std::fs;

fn main() {
    let input: Vec<(u16, u16, u16)> = fs::read_to_string("day3/resources/input.txt")
        .expect("read input file")
        .lines()
        .map(|s| {
            return (
                s[..5].trim().parse().expect("parsed number"),
                s[5..10].trim().parse().expect("parsed number"),
                s[10..].trim().parse().expect("parsed number"),
            );
        })
        .collect();

    println!(
        "{}",
        input
            .iter()
            .filter(|triangle| is_possible(*triangle))
            .count()
    );

    let transposed = transpose(&input);

    println!(
        "{}",
        transposed
            .iter()
            .filter(|triangle| is_possible(*triangle))
            .count()
    );
}

fn is_possible(triangle: &(u16, u16, u16)) -> bool {
    return triangle.0 + triangle.1 > triangle.2
        && triangle.1 + triangle.2 > triangle.0
        && triangle.0 + triangle.2 > triangle.1;
}

fn transpose(input: &Vec<(u16, u16, u16)>) -> Vec<(u16, u16, u16)> {
    let mut row = 0;
    let mut result = Vec::new();

    while row < input.len() {
        result.push((
            input.get(row).unwrap().0,
            input.get(row + 1).unwrap().0,
            input.get(row + 2).unwrap().0,
        ));
        result.push((
            input.get(row).unwrap().1,
            input.get(row + 1).unwrap().1,
            input.get(row + 2).unwrap().1,
        ));
        result.push((
            input.get(row).unwrap().2,
            input.get(row + 1).unwrap().2,
            input.get(row + 2).unwrap().2,
        ));
        row += 3;
    }

    return result;
}
