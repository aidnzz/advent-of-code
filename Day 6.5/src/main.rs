#![feature(array_windows)]

fn main() {
    const INPUT: &[u8] = include_bytes!("input.txt");
    
    let position = INPUT
        .windows(4)
        .position(|elements| {
            !elements
                .iter()
                .any(|e| elements.iter().filter(|&n| e == n).count() > 1)
        })
        .map(|n| n + 4);
    
    println!("{}", position.unwrap());
}
