fn main() {
    let example = "..^^.";

    let input = example;
    let map_rows = 4;

    let mut map = vec![input.chars().collect::<Vec<_>>()];

    while map.len() < map_rows {
        map.push(calculate_row(&map[map.len() - 1]));
    }

    println!("{}", count_safe_tiles(&map));
}

fn count_safe_tiles(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .map(|row| row.iter().filter(|c| **c == '.').count())
        .sum()
}

fn calculate_row(row: &[char]) -> Vec<char> {
    let mut result = Vec::new();

    for i in 0..row.len() {
        let left = if i > 0 { row[i - 1] } else { '.' };
        let center = row[i];
        let right = if i < row.len() - 1 { row[i + 1] } else { '.' };

        if (left == '^' && center == '^' && right == '.')
            || (left == '.' && center == '^' && right == '^')
            || (left == '^' && center == '.' && right == '.')
            || (left == '.' && center == '.' && right == '^')
        {
            result.push('^');
        } else {
            result.push('.');
        }
    }

    result
}
