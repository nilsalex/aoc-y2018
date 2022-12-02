use multimap::MultiMap;
use regex::Regex;

pub const INPUT: &str = include_str!("input.txt");

struct Window {
    begin: u8,
    end: u8,
}

fn parse_input(input: &str) -> MultiMap<usize, Window> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut window_map = MultiMap::<usize, Window>::new();

    let guard_regex = Regex::new(r"(\d+)-(\d+)-(\d+) (\d+):(\d+).+#(\d+)").unwrap();
    let minute_regex = Regex::new(r":(\d+)").unwrap();

    lines.sort();
    let mut lines_it = lines.iter().peekable();

    while lines_it.peek().is_some() {
        let capture = guard_regex.captures_iter(lines_it.next().unwrap()).next().unwrap();
        let guard = capture[6].parse::<usize>().unwrap();
        while lines_it.peek().map(|l| !l.contains('#')) == Some(true) {
            let capture_begin = minute_regex.captures_iter(lines_it.next().unwrap()).next().unwrap();
            let capture_end = minute_regex.captures_iter(lines_it.next().unwrap()).next().unwrap();
            window_map.insert(guard, Window { begin: capture_begin[1].parse::<u8>().unwrap(), end: capture_end[1].parse::<u8>().unwrap() })
        }
    }

    window_map
}

fn part1(input: &str) -> usize {
    let window_map = parse_input(input);

    let sums = window_map.
        iter_all()
        .map(|(guard, windows)| (*guard, windows.iter().map(|w| w.end as usize - w.begin as usize).sum()))
        .collect::<Vec<(usize, usize)>>();

    let max_guard = sums.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;

    let mut minute_counts = [0_usize; 60];

    for window in window_map.get_vec(&max_guard).unwrap().iter() {
        for i in window.begin..window.end {
            minute_counts[i as usize] += 1
        }
    };

    minute_counts.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().0 * max_guard
}

fn part2(input: &str) -> usize {
    let window_map = parse_input(input);

    let mut max_guard: usize = 0;
    let mut max_minute: usize = 0;
    let mut max_minute_count: usize = 0;

    for (guard, windows) in window_map {
        let mut minute_counts = [0_usize; 60];

        for window in windows {
            for i in window.begin..window.end {
                minute_counts[i as usize] += 1
            }
        };

        let (minute, minute_count) = minute_counts.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

        if minute_count > &max_minute_count {
            max_guard = guard;
            max_minute = minute;
            max_minute_count = *minute_count;
        }
    };

    max_guard * max_minute
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
