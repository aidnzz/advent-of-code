#![feature(array_windows)]

fn main() {
    const INPUT: &[u8] = include_bytes!("input.txt");
    
    for (index, elements) in INPUT.array_windows::<4>().enumerate() {
        let duplicate = elements.iter().any(|e| elements.iter().filter(|&n| e == n).count() > 1);
        if !duplicate {
            println!("First marker at: {}", index + 4);
            break;
        }
    }
}
