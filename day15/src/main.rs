#[derive(Debug)]
struct Disc {
    number_of_positions: u8,
    start_position: u8,
}

fn main() {
    let discs = vec![
        Disc {
            number_of_positions: 5,
            start_position: 4,
        },
        Disc {
            number_of_positions: 2,
            start_position: 1,
        },
    ];

    let mut push_at_time = 0;

    while !is_valid_push_time(&discs, push_at_time) {
        push_at_time += 1;
    }

    println!("{}", push_at_time);
}

fn is_valid_push_time(discs: &Vec<Disc>, push_time: usize) -> bool {
    for disc_index in 0..discs.len() {
        let position_at_time =
            calculate_position_at_time(&discs[disc_index], push_time + 1 + disc_index);

        if position_at_time != 0 {
            return false;
        }
    }

    true
}

fn calculate_position_at_time(disc: &Disc, time: usize) -> usize {
    (disc.start_position as usize + time) % disc.number_of_positions as usize
}
