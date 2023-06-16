use std::{fs::read_to_string, collections::BinaryHeap, cmp::Reverse};

fn get_calories_from_file(path: &str) -> Vec<u32> {
    match read_to_string(path) {
        Ok(str) => {
            let mut result: Vec<u32> = vec![];
            let mut amount: u32 = 0;
            str.lines().for_each(|line| {
                if line.is_empty() {
                    result.push(amount);
                    amount = 0;
                } else {
                    amount += line.parse::<u32>().unwrap();
                }
            });

            result
        }
        Err(_) => vec![],
    }
}

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

    println!("Part 2: {:?}", heap.into_iter().map(|rev| rev.0).sum::<u32>());
}

fn main() {
    let elf_carry_load = get_calories_from_file("./test_files/day_1.txt");
    part_1(&elf_carry_load);
    part_2(&elf_carry_load);
}

#[test]
fn test_part_1() {
    assert_eq!(
        &24000,
        get_calories_from_file("./test_files/day_1_test.txt")
            .iter()
            .max()
            .unwrap()
    );
}
