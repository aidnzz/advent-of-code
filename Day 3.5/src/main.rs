use itertools::{iproduct, Itertools};

struct Group<'a>(&'a str, &'a str, &'a str);

impl<'a> Group<'a> {
    fn common(&self) -> Option<u8> {
        // 0(n^3)
        for (a, b, c) in iproduct!(self.0.as_bytes(), self.1.as_bytes(), self.2.as_bytes()) {
            let equal = [a, b, c].iter().all(|&i| i == a);

            if equal {
                return Some(*a);
            }
        }
        None
    }
}

trait Priority {
    fn priority(&self) -> u32;
}

impl Priority for u8 {
    fn priority(&self) -> u32 {
        if self.is_ascii_lowercase() {
            (*self - b'a' + 1) as u32
        } else {
            (*self - b'A' + 27) as u32
        }
    }
}

fn main() {
    const LINES: &str = include_str!("input.txt");

    let mut total: u32 = 0;

    for mut chunk in &LINES.lines().chunks(3) {
        let group = Group(
            chunk.next().unwrap(),
            chunk.next().unwrap(),
            chunk.next().unwrap(),
        );

        total += group.common().unwrap().priority()
    }

    println!("Total priority: {total}");
}
