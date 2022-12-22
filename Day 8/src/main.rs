use std::mem::MaybeUninit;

const INPUT: &str = include_str!("input.txt");

#[derive(Default)]
struct Position<const R: usize, const C: usize> {
    x: usize,
    y: usize,
}

impl<const R: usize, const C: usize> Position<R, C> {
    #[inline]
    fn within(&self) -> bool {
        (1..R - 1).contains(&self.y) || (1..C - 1).contains(&self.x)
    }

    #[inline]
    fn on_edge(&self) -> bool {
        !self.within()
    }

    fn surroundings(&self) -> [Vec<Self>; 4] {
        [
            // above
            (0..self.y).map(|y| Self { x: self.x, y }).collect(),
            // below
            (self.y + 1..C).map(|y| Self { x: self.x, y }).collect(),
            // left
            (0..self.x).map(|x| Self { x, y: self.y }).collect(),
            // right
            (self.x + 1..R).map(|x| Self { x, y: self.y }).collect(),
        ]
    }
}

struct Tree<const R: usize, const C: usize> {
    height: u32,
    position: Position<R, C>,
}

struct Forest<const R: usize, const C: usize>([[u32; R]; C]);

impl<const R: usize, const C: usize> Forest<R, C> {
    #[inline]
    fn get(&self, &Position { x, y }: &Position<R, C>) -> u32 {
        self.0[y][x]
    }

    fn trees(&self) -> Trees<R, C> {
        let it = self.0.iter().enumerate().flat_map(|(y, column)| {
            column.iter().enumerate().map(move |(x, &height)| Tree {
                height,
                position: Position { x, y },
            })
        });
        Trees(Box::new(it))
    }
}

unsafe fn parse_forest<const R: usize, const C: usize>(input: &str) -> Forest<R, C> {
    let mut buffer = MaybeUninit::<[[u32; R]; C]>::uninit();

    for (raw, row)  in buffer.assume_init_mut().iter_mut().zip(input.lines()) {
        *raw = row.chars().map(|f| f.to_digit(10).unwrap_unchecked()).collect::<Vec<u32>>().try_into().unwrap();
    }

    Forest(buffer.assume_init())
}

struct Trees<'a, const R: usize, const C: usize>(Box<dyn Iterator<Item = Tree<R, C>> + 'a>);

impl<const R: usize, const C: usize> Iterator for Trees<'_, R, C> {
    type Item = Tree<R, C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

fn main() {
    let forest: Forest<99, 99> = unsafe { parse_forest(INPUT) };

    let total = forest.trees().filter(|tree| {
        let tallest = tree.position.surroundings().iter().any(|neighbours| {
            neighbours.iter().all(|p| tree.height > forest.get(p))
        });
        
        tree.position.on_edge() || tallest
    }).count();
 
    println!("Tallest: {total}")
}   