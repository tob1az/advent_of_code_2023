fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let first = l.chars().find(|c| c.is_numeric()).unwrap();
            let last = l.chars().rev().find(|c| c.is_numeric()).unwrap();
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum()
}

const NUMBERS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let first = NUMBERS
                .iter()
                .enumerate()
                .filter_map(|(i, n)| l.find(n).and_then(|p| Some((i, p))))
                .min_by_key(|(_, p)| *p)
                .unwrap()
                .0;
            let last = NUMBERS
                .iter()
                .enumerate()
                .filter_map(|(i, n)| l.rfind(n).and_then(|p| Some((i, p))))
                .max_by_key(|(_, p)| *p)
                .unwrap()
                .0;
            let n = format!("{}{}", NUMBERS[first % 9], NUMBERS[last % 9])
                .parse::<usize>()
                .unwrap();
            dbg!(&n);
            n
        })
        .sum()
}

#[common::solution]
fn calculate_solution(input: &str) -> (usize, usize) {
    (part1(input), part2(input))
}
