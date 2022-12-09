#![feature(array_windows)]

fn main() {
    const INPUT: &[u8] = include_bytes!("input.txt");
    
    for (index, elements) in INPUT.array_windows::<14>().enumerate() {
        let duplicate = elements.iter().any(|e| elements.iter().filter(|&n| e == n).count() > 1);
        if !duplicate {
            println!("First marker at: {}", index + 14);
            break;
        }
    }
}
