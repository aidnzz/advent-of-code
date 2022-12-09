fn main() {
    const INPUT: &[u8] = include_bytes!("input.txt");
    
    let position = INPUT
        .windows(14)
        .position(|elements| {
            !elements
                .iter()
                .any(|e| elements.iter().filter(|&n| e == n).count() > 1)
        })
        .map(|n| n + 14);
    
    println!("{}", position.unwrap());
}
