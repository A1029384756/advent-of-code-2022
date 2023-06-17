use std::fs::read_to_string;

fn find_common_item_in_bag(bag: &str) -> Option<char> {
    let (first_compartment, second_compartment) = bag.split_at(bag.len() / 2);
    for c in first_compartment.chars() {
        if second_compartment.contains(c) {
            return Some(c);
        }
    }

    None
}

fn get_priority_from_char(c: char) -> Result<u32, ()> {
    if c as u8 >= 65 && c as u8 <= 90 {
        Ok((c as u8 - 38) as u32)
    } else if c as u8 >= 97 && c as u8 <= 122 {
        Ok((c as u8 - 96) as u32)
    } else {
        Err(())
    }
}

fn part_1(compartments: Vec<&str>) -> u32 {
    compartments
        .iter()
        .map(|compartment| match find_common_item_in_bag(compartment) {
            Some(item) => match get_priority_from_char(item) {
                Ok(priority) => priority,
                Err(_) => panic!("Invalid char outside of boundary"),
            },
            None => panic!("No matching item in rucksack"),
        })
        .sum()
}

fn part_2(compartments: Vec<&str>) -> u32 {
    compartments
        .chunks(3)
        .map(|group| {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    return match get_priority_from_char(c) {
                        Ok(priority) => priority,
                        Err(_) => panic!("Invalid char outside of boundary"),
                    };
                }
            }

            panic!("Group without common item");
        })
        .sum()
}

fn main() {
    let file = read_to_string("./test_files/day_3.txt").unwrap();
    let input: Vec<&str> = file.lines().collect();
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
}

#[test]
fn part_1_test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(part_1(input.lines().collect()), 157);
}
