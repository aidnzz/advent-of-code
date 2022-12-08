#![feature(iter_advance_by)]

type Item = u8;

#[derive(Debug)]
struct Stack([Vec<Item>; Table::WIDTH]);

struct Table<'a>(Vec<&'a str>);

impl<'a> Table<'a> {
    const WIDTH: usize = 9;

    fn new(s: &'a str) -> Self {
        let mut lines: Vec<&'a str> = s.lines().collect();
        lines.pop(); // Ignore index line

        Self(lines)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn max_elements(&self) -> usize {
        // Maximum elements a stack can have
        self.height() * Self::WIDTH
    }
}

impl Stack {
    fn new(table: &Table) -> Self {
        let mut stacks: [Vec<Item>; Table::WIDTH] = Default::default();

        let max_height = table.max_elements();
        stacks.iter_mut().for_each(|s| s.reserve(max_height));

        for i in table.0.iter().rev() {
            for (index, item) in i.bytes().skip(1).step_by(4).enumerate() {
                if item != b' ' {
                    stacks[index].push(item);
                }
            }
        }

        Self(stacks)
    }

    fn translate_move(&mut self, s: &str) {
        let (quantity, from, to) = (
            s.split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            s.split_ascii_whitespace()
                .nth(3)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            s.split_ascii_whitespace()
                .nth(5)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );

        for _ in 0..quantity {
            let value = self.0[from - 1].pop().unwrap();
            self.0[to - 1].push(value);
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let (map, moves) = INPUT.split_once("\n\n").unwrap();

    let mut stack = Stack::new(&Table::new(map));
    moves.lines().for_each(|s| stack.translate_move(s));

    for row in stack.0.iter() {
        print!("{}", *row.last().unwrap() as char);
    }
}
