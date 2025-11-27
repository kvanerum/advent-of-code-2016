use std::collections::VecDeque;
use std::fs;

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let input = fs::read_to_string("day24/resources/input.txt").expect("failed to read input file");
    let (part1, part2) = solve(&input);

    println!("fewest steps = {}", part1);
    println!("fewest steps with return = {}", part2);
}

fn solve(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut numbered_points = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value.is_ascii_digit() {
                numbered_points.push((*value, Point { x, y }));
            }
        }
    }
    numbered_points.sort_by_key(|entry| entry.0);

    let nodes: Vec<Point> = numbered_points.iter().map(|(_, point)| *point).collect();
    let start_index = numbered_points
        .iter()
        .position(|(label, _)| *label == '0')
        .expect("missing starting position");

    let distances = compute_pairwise_distances(&grid, &nodes);
    tsp(&distances, start_index)
}

fn compute_pairwise_distances(grid: &Vec<Vec<char>>, nodes: &[Point]) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; nodes.len()]; nodes.len()];

    for (idx, &start) in nodes.iter().enumerate() {
        let distance_map = bfs(grid, start);
        for (target_idx, &target) in nodes.iter().enumerate() {
            result[idx][target_idx] = distance_map[target.y][target.x];
        }
    }

    result
}

fn bfs(grid: &Vec<Vec<char>>, start: Point) -> Vec<Vec<usize>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut distances = vec![vec![usize::MAX; width]; height];
    let mut queue = VecDeque::new();
    let steps = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];

    distances[start.y][start.x] = 0;
    queue.push_back(start);

    while let Some(point) = queue.pop_front() {
        let current_distance = distances[point.y][point.x];

        for (dx, dy) in steps.iter() {
            let nx = point.x as isize + dx;
            let ny = point.y as isize + dy;

            if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                continue;
            }

            let (nxu, nyu) = (nx as usize, ny as usize);
            if grid[nyu][nxu] == '#' {
                continue;
            }

            if distances[nyu][nxu] != usize::MAX {
                continue;
            }

            distances[nyu][nxu] = current_distance + 1;
            queue.push_back(Point { x: nxu, y: nyu });
        }
    }

    distances
}

fn tsp(distances: &Vec<Vec<usize>>, start_index: usize) -> (usize, usize) {
    let count = distances.len();
    let full_mask = (1usize << count) - 1;
    let mut dp = vec![vec![usize::MAX; count]; 1 << count];

    dp[1 << start_index][start_index] = 0;

    for mask in 0..=full_mask {
        for current in 0..count {
            if dp[mask][current] == usize::MAX {
                continue;
            }

            for next in 0..count {
                if mask & (1 << next) != 0 {
                    continue;
                }

                let distance = distances[current][next];
                if distance == usize::MAX {
                    continue;
                }
                let next_mask = mask | (1 << next);
                let candidate = dp[mask][current] + distance;
                if candidate < dp[next_mask][next] {
                    dp[next_mask][next] = candidate;
                }
            }
        }
    }

    let part1 = dp[full_mask]
        .iter()
        .copied()
        .filter(|value| *value != usize::MAX)
        .min()
        .expect("no path found");

    let mut part2 = usize::MAX;
    for (idx, &cost) in dp[full_mask].iter().enumerate() {
        if cost == usize::MAX {
            continue;
        }

        let back = distances[idx][start_index];
        if back == usize::MAX {
            continue;
        }

        part2 = part2.min(cost + back);
    }

    (part1, part2)
}
