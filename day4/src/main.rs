use std::collections::HashMap;
use std::fs;

struct Room {
    name: String,
    id: u16,
    checksum: String,
}

impl Room {
    pub fn new(input: &str) -> Self {
        let checksum_start = input.find('[').expect("find checksum");
        let id_start = input.rfind('-').expect("find id");

        let checksum = &input[checksum_start + 1..input.len() - 1];
        let id = &input[id_start + 1..checksum_start];

        Self {
            checksum: checksum.to_string(),
            id: id.parse().expect("id"),
            name: input[0..id_start].to_string(),
        }
    }

    pub fn is_real(&self) -> bool {
        return self.checksum == self.calculate_checksum()[0..5];
    }

    fn calculate_checksum(&self) -> String {
        let letter_map = self.get_letter_map();
        let mut as_vector: Vec<_> = letter_map.iter().collect();
        as_vector.sort_by(|a, b| a.1.cmp(b.1).reverse().then(a.0.cmp(b.0)));

        return as_vector.iter().map(|x| x.0).collect();
    }

    fn get_letter_map(&self) -> HashMap<char, u8> {
        let mut letters: HashMap<char, u8> = HashMap::new();

        for c in self.name.chars() {
            if c != '-' {
                *letters.entry(c).or_insert(0) += 1;
            }
        }

        return letters;
    }

    fn decrypt(&self) -> String {
        return self
            .name
            .chars()
            .into_iter()
            .map(|c| self.shift(c))
            .collect();
    }

    fn shift(&self, c: char) -> char {
        if c == '-' {
            return ' ';
        }

        let a = 'a' as u32;
        let char_index = ((c as u32 - a) + self.id as u32) % 26;

        return char::from_u32(a + char_index).expect("char");
    }
}

fn main() {
    let input: Vec<Room> = fs::read_to_string("day4/resources/input.txt")
        .expect("read input file")
        .lines()
        .map(|s| Room::new(s))
        .collect();

    let real_rooms: Vec<_> = input.iter().filter(|room| room.is_real()).collect();

    let sum: u32 = real_rooms.iter().map(|room| room.id as u32).sum();

    println!("{}", sum);

    real_rooms
        .iter()
        .for_each(|room| println!("{}: {}", room.id, room.decrypt()))
}
