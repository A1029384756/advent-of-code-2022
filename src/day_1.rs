use std::fs::read_to_string;

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

fn part_1(elf_carry_load: Vec<u32>) {
    println!(
        "{}",
        get_calories_from_file("./test_files/day_1.txt")
            .iter()
            .max()
            .unwrap()
    );
}

fn main() {}

#[cfg(test)]
fn test_part_1() {
    assert_eq!(
        &11000,
        get_calories_from_file("./test_files/day_1.txt")
            .iter()
            .max()
            .unwrap()
    );
}
