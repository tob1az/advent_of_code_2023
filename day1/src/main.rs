#[common::solution]
fn calculate_solution(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let first = l.chars().find(|c| c.is_numeric()).unwrap();
            let last = l.chars().rev().find(|c| c.is_numeric()).unwrap();
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum()
}
