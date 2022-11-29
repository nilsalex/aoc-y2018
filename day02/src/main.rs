pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    let mut pair_count: usize = 0;
    let mut triple_count: usize = 0;

    input
        .lines()
        .for_each(|l|
            {
                let mut bytes = l.as_bytes().to_owned();
                bytes.sort();
                let mut counts: Vec<u8> = Vec::new();
                counts.push(1);
                let mut prev: u8 = 0;
                for b in bytes {
                    if b == prev {
                        *counts.last_mut().unwrap() += 1
                    } else {
                        counts.push(1);
                        prev = b
                    }
                }

                if counts.contains(&2) {
                    pair_count += 1
                }
                if counts.contains(&3) {
                    triple_count += 1
                }
            });
    pair_count * triple_count
}

fn part2(input: &str) -> String {
    let input_lines: Vec<&str> = input.lines().collect();
    for i in 0..input_lines.len()-1 {
        for j in i+1..input_lines.len() {
            let id1: Vec<u8> = input_lines[i].as_bytes().to_owned();
            let id2: Vec<u8> = input_lines[j].as_bytes().to_owned();
            let mut diffs_count: u8 = 0;
            let mut result: Vec<u8> = Vec::new();

            for k in 0..id1.len() {
              if id1[k] != id2[k] {
                  diffs_count += 1
              } else {
                  result.push(id1[k])
              }
            }

            if diffs_count == 1 {
                return String::from_utf8(result).unwrap();
            }
        }
    }

    panic!()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
