use std::{cmp::Reverse, collections::BinaryHeap};

fn part_1(elf_carry_load: &Vec<u32>) {
    println!("Part 1: {}", elf_carry_load.iter().max().unwrap());
}

fn part_2(elf_carry_load: &Vec<u32>) {
    let mut heap = BinaryHeap::new();
    for item in elf_carry_load.iter() {
        heap.push(Reverse(item));
        if heap.len() > 3 {
            heap.pop();
        }
    }

    println!(
        "Part 2: {:?}",
        heap.into_iter().map(|rev| rev.0).sum::<u32>()
    );
}

fn main() {
    let elf_carry_load = include_str!("test_files/day_1.txt")
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|group| group.iter().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect();
    part_1(&elf_carry_load);
    part_2(&elf_carry_load);
}

#[test]
fn test_part_1() {
    let elf_carry_load: Vec<u32> = include_str!("test_files/day_1_test.txt")
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|group| group.iter().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect();

    assert_eq!(&24000, elf_carry_load.iter().max().unwrap());
}
