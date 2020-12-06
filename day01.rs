use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TARGET: i32 = 2020;

fn main() {
    if let Ok(lines) = read_lines("./01.txt") {
        let numbers: Vec<_> = lines
            .map(|x| x.expect("could not parse").parse::<i32>().unwrap())
            .collect();
        solve1(&numbers, TARGET);
        solve2(&numbers);
    }
}

fn solve1(v: &std::vec::Vec<i32>, target: i32) -> i32 {
    for n in v {
        if v.contains(&(target - n)) {
            println!(
                "{} + {} = {}, multiplied equals {}",
                n,
                target - n,
                target,
                n * (target - n)
            );
            return n * (target - n);
        }
    }
    return -1;
}

fn solve2(v: &std::vec::Vec<i32>) -> i32 {
    for n in v {
        let y = solve1(v, TARGET - n);
        if y != -1 {
            println!("part2 is {} * {} = {}", y, n, y * n);
            return y * n;
        }
    }
    return -1;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
