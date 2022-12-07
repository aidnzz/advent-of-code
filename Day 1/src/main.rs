fn main() {
    let input = include_str!("file.txt");

    let lines: Vec<_> = input.lines().map(|s| s.parse::<u32>().ok()).collect();
    
    let largest = lines
        .split(Option::is_none)
        .map(|g| g.iter().map(|v| v.unwrap()).max().unwrap())                    
        .max()
        .unwrap();

    println!("Largest: {largest}");
}