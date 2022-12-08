use std::ops::RangeInclusive;

struct Section(RangeInclusive<u32>, RangeInclusive<u32>);

impl From<&str> for Section {
    fn from(s: &str) -> Self {
        let digits = s
            .split(['-', ','])
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if let &[w, x, y, z] = digits.as_slice() {
            Self(w..=x, y..=z)
        } else {
            unreachable!()
        }
    }
}

impl Section {
    fn overlap(&self) -> bool {
        self.0.contains(self.1.start()) || self.0.contains(self.1.end())
            || self.1.contains(self.0.start()) || self.1.contains(self.0.end())
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");

    let total = INPUT
        .lines()
        .filter(|&l| Section::from(l).overlap())
        .count();

    println!("Total overlapped: {total}");
}
