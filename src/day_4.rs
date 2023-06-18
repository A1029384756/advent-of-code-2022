use std::fs::read_to_string;

use anyhow::Result;

fn get_sections(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            let sections: Vec<&str> = line.split(',').collect();
            sections
                .iter()
                .map(|section| section.split('-'))
                .flatten()
                .map(|section| {
                    section
                        .parse::<u32>()
                        .expect("There should always be a number here")
                })
                .collect()
        })
        .collect()
}

fn part_1(sections: &Vec<Vec<u32>>) -> u32 {
    sections
        .iter()
        .map(|section| {
            (((section[1] >= section[3]) && (section[0] <= section[2]))
                || ((section[1] <= section[3]) && (section[0] >= section[2]))) as u32
        })
        .sum()
}

fn part_2(sections: &Vec<Vec<u32>>) -> u32 {
    sections
        .iter()
        .map(|section| {
            ((section[0] <= section[3]) && (section[1] >= section[2])) as u32
        })
        .sum()
}

fn main() -> Result<()> {
    let file = &read_to_string("./test_files/day_4.txt").unwrap();
    let sections = get_sections(file);

    println!("Part 1: {}", part_1(&sections));
    println!("Part 2: {}", part_2(&sections));
    Ok(())
}

#[test]
fn test_part_1() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    assert_eq!(part_1(&get_sections(input)), 2);
}

#[test]
fn test_part_2() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    assert_eq!(part_2(&get_sections(input)), 4);
}
