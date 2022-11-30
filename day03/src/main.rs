use regex::Regex;

pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    let re = Regex::new(r"#\d* @ (\d*),(\d*): (\d*)x(\d*)").unwrap();
    let mut grid: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
    let mut result: usize = 0;

    input
        .lines()
        .for_each(|l| {
            let captures = re.captures(l).unwrap();

            let begin_x: usize = captures[1].parse().unwrap();
            let begin_y: usize = captures[2].parse().unwrap();
            let width_x: usize = captures[3].parse().unwrap();
            let width_y: usize = captures[4].parse().unwrap();

            for x in begin_x..begin_x+width_x {
                for y in begin_y..begin_y+width_y {
                    let count = grid.get_mut(x).unwrap().get_mut(y).unwrap();
                    *count += 1;
                    if *count == 2 {
                        result += 1
                    }
                }
            }
        });
    result
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"#\d* @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

    let claims: Vec<[usize; 4]> = input
        .lines()
        .map(|l| {
            let captures = re.captures(l).unwrap();

            let begin_x: usize = captures[1].parse().unwrap();
            let begin_y: usize = captures[2].parse().unwrap();
            let width_x: usize = captures[3].parse().unwrap();
            let width_y: usize = captures[4].parse().unwrap();

            [begin_x, begin_y, width_x, width_y]
        })
        .collect();

    let mut grid: [[u8; 1000]; 1000] = [[0; 1000]; 1000];

    for claim in claims.iter() {
        let begin_x = claim[0];
        let begin_y = claim[1];
        let width_x = claim[2];
        let width_y = claim[3];

        for x in begin_x..begin_x+width_x {
            for y in begin_y..begin_y+width_y {
                let count = grid.get_mut(x).unwrap().get_mut(y).unwrap();
                *count += 1;
            }
        }
    }

    'outer: for (i, claim) in claims.iter().enumerate() {
        let begin_x = claim[0];
        let begin_y = claim[1];
        let width_x = claim[2];
        let width_y = claim[3];

        for row in grid.iter().skip(begin_x).take(width_x) {
            for elem in row.iter().skip(begin_y).take(width_y) {
                if *elem > 1 {
                    continue 'outer;
                }
            }
        }

        return i+1;
    }

    panic!()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
