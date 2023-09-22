use md5;
use std::collections::HashMap;

fn main() {
    let input = "cxdnnyjw";
    let mut chars_generated: u8 = 0;
    let mut i: u32 = 0;

    while chars_generated < 8 {
        let md5 = md5::compute(input.clone().to_owned() + &i.to_string());
        let starts_with_zeroes = md5.0[0] == 0 && md5.0[1] == 0 && (md5.0[2] & 0b11110000) == 0;

        if starts_with_zeroes {
            chars_generated += 1;
            print!("{:x?}", md5.0[2]);
        }

        i += 1;
    }

    println!();

    i = 0;
    let mut result_map: HashMap<u8, u8> = HashMap::new();

    while result_map.len() < 8 {
        let md5 = md5::compute(input.clone().to_owned() + &i.to_string());
        let starts_with_zeroes = md5.0[0] == 0 && md5.0[1] == 0 && (md5.0[2] & 0b11110000) == 0;
        let position = md5.0[2];
        let char = md5.0[3] >> 4;

        if starts_with_zeroes && md5.0[2] < 8 && !result_map.contains_key(&position) {
            result_map.insert(position, char);
        }

        i += 1;
    }

    for x in 0..8 {
        print!("{:x?}", result_map.get(&x).expect("char found"));
    }
}
