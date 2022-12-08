struct Rucksack<'a>(&'a str, &'a str);

impl<'a> Rucksack<'a> {
    fn common(&self) -> Option<u8> {
        for x in self.0.as_bytes() {
            for y in self.1.as_bytes() {
                if x == y {
                    return Some(*x);
                }
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

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(s: &'a str) -> Self {
        let (x, y) = s.split_at(s.len() / 2);
        Self(x, y)
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let total: u32 = INPUT
        .lines()
        .map(|l| Rucksack::from(l).common().unwrap().priority())
        .sum();
    println!("Total: {total}");
}
