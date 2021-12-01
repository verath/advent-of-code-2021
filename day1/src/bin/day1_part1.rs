fn day1_part1(input: &str) -> i64 {
    let ls: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut larger = 0;
    for i in 1..ls.len() {
        if ls[i - 1] < ls[i] {
            larger += 1
        }
    }
    larger
}

fn main() {
    let input = day1::INPUT.trim_end();
    println!("{:?}", day1_part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part2_example() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(day1_part1(input), 7)
    }

    #[test]
    fn test_solve_day1_part1() {
        let input = day1::INPUT.trim_end();
        assert_eq!(day1_part1(input), 1709);
    }
}
