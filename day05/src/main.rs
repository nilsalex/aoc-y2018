#![feature(byte_slice_trim_ascii)]

pub const INPUT: &[u8] = include_bytes!("input.txt");

fn reduce_polymer<'a>(polymer: impl Iterator<Item=&'a u8>, result: &mut Vec<u8>) -> usize
{
    polymer.for_each(|unit|
        if result.is_empty() {
            result.push(*unit);
        } else if result.last().unwrap().abs_diff(*unit) == 32 {
            result.pop();
        } else {
            result.push(*unit);
        });

    result.len()
}

fn part1(input: &[u8]) -> usize {
    let polymer = input.trim_ascii_end();
    let mut result = Vec::with_capacity(polymer.len());

    reduce_polymer(polymer.iter(), &mut result)
}

fn part2(input: &[u8]) -> usize {
    let polymer = input.trim_ascii_end();
    let mut result = Vec::with_capacity(polymer.len());

    (0_u8..26_u8).map(|unit_drop|
        {
            result.clear();
            let filtered_polymer = polymer.iter().filter(|unit| **unit != unit_drop + 65 && **unit != unit_drop + 97);
            reduce_polymer(filtered_polymer, &mut result)
        }
    ).min().unwrap()
}

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}
