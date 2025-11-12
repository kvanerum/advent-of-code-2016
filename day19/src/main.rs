use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

struct Elf {
    next_elf_id: usize,
    previous_elf_id: usize,
}

fn main() {
    let input = 5;

    part1(input);
    part2(input);
}

fn part2(input: usize) {
    let mut elves = Vec::with_capacity(input);

    (1..=input).for_each(|elf_id| elves.push(elf_id));

    let mut current_elf_id = 0;

    let pb = ProgressBar::new(input as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {per_sec}")
            .unwrap()
            .progress_chars("#>-"),
    );

    while elves.len() > 1 {
        let steal_from_elf_index = (current_elf_id + elves.len() / 2) % elves.len();

        elves.remove(steal_from_elf_index);

        if steal_from_elf_index > current_elf_id {
            current_elf_id += 1
        }

        current_elf_id %= elves.len();

        pb.inc(1);
    }

    pb.finish_and_clear();

    println!("{}", elves.get(0).unwrap());
}

fn part1(input: usize) {
    let mut elves = HashMap::new();

    (1..=input).for_each(|elf_id| {
        elves.insert(
            elf_id,
            Elf {
                next_elf_id: if elf_id == input { 1 } else { elf_id + 1 },
                previous_elf_id: if elf_id == 1 { input } else { elf_id - 1 },
            },
        );
    });

    let mut current_elf_id = 1;

    while elves.len() > 1 {
        let steal_from_elf_id = elves.get(&current_elf_id).unwrap().next_elf_id;
        let steal_from_elf_previous_id = elves.get(&steal_from_elf_id).unwrap().previous_elf_id;
        let steal_from_elf_next_id = elves.get(&steal_from_elf_id).unwrap().next_elf_id;

        elves
            .get_mut(&steal_from_elf_previous_id)
            .unwrap()
            .next_elf_id = steal_from_elf_next_id;

        elves
            .get_mut(&steal_from_elf_next_id)
            .unwrap()
            .previous_elf_id = steal_from_elf_previous_id;

        elves.remove(&steal_from_elf_id);

        current_elf_id = elves.get(&current_elf_id).unwrap().next_elf_id;
    }

    println!("{}", current_elf_id);
}
