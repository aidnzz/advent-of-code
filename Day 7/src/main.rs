#![feature(option_result_contains)]
    
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
struct Directory<'a> {
    path: &'a Path,
    files: &'a Vec<(&'a str, usize)>,
    size: usize,
    directories: Vec<Directory<'a>>,
}

fn build_directory<'a>(
    root: &'a Path,
    fs: &'a HashMap<PathBuf, Vec<(&str, usize)>>,
) -> Directory<'a> {
    let files = fs.get(root).unwrap();

    let mut root_directory = Directory {
        files,
        path: root,
        size: files.iter().map(|(_name, size)| size).sum(),
        directories: Default::default(),
    };

    for path in fs.keys() {
        // Check if parent path equals root
        if path.parent().contains(&root) {
            let directory = build_directory(path, fs);
            root_directory.size += directory.size;
            root_directory.directories.push(directory);
        }
    }

    root_directory
}

fn solution(root: &Directory) -> usize {
    let mut total = if root.size <= 100_000 { root.size } else { 0 };

    for folder in root.directories.iter() {
        total += solution(folder);
    }

    total
}

fn main() {
    let mut current_directory = PathBuf::new();
    let mut flat_file_system = HashMap::<PathBuf, Vec<(&str, usize)>>::new();

    for line in INPUT.lines() {
        let command: Vec<&str> = line.split_ascii_whitespace().collect();

        match *command.as_slice() {
            ["$", "cd", path] => {
                current_directory = match path {
                    "/" => PathBuf::new(),
                    ".." => current_directory.parent().unwrap().to_owned(),
                    name => current_directory.join(name),
                };
            }
            ["dir", name] => {
                let path = current_directory.join(name);
                flat_file_system.entry(path).or_default();
            }
            ["$", "ls"] => {}
            [size, name] => {
                let file_size = size.parse::<usize>().unwrap();
                flat_file_system
                    .entry(current_directory.clone())
                    .or_default()
                    .push((name, file_size));
            }
            _ => {}
        }
    }

    let root = Path::new("");
    let tree = build_directory(root, &flat_file_system);

    let total = solution(&tree);
    println!("{total}");
}
