pub const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .sum()
}

fn part2(input: &str) -> i64 {
    let nums: Vec<i64> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut visited: [bool; 1024*1024] = [false; 1024*1024];
    let mut cur: i64 = 0;

    loop {
        for num in &nums {
            cur += num;
            let ix = if cur < 0 {
                (cur + 512*1024) as usize
            } else {
                cur as usize
            };
            if visited[ix] {
                return cur;
            }
            visited[ix] = true
        }
    }
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
