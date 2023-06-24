use std::collections::HashSet;

fn find_marker(input: &str, n: usize) -> usize {
    input
        .as_bytes()
        .windows(n)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == n)
        .unwrap()
        + n
}

fn part_1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part_2(input: &str) -> usize {
    find_marker(input, 14)
}

fn main() {
    let input = include_str!("test_files/day_6.txt");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
fn test_part_1() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part_1(input), 7);

    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part_1(input), 5);

    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part_1(input), 6);

    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part_1(input), 10);

    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_1(input), 11);
}

#[test]
fn test_part_2() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part_2(input), 19);

    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part_2(input), 23);

    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part_2(input), 23);

    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part_2(input), 29);

    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_2(input), 26);
}
