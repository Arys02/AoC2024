use std::collections::HashMap;

fn check_safe(report: &str) -> usize {
    let report : Vec<usize>= report.split(" ").flat_map(|x| { x.parse::<usize>()}).collect();

    let is_desc = report[0] > report[1];

    for i in 1..report.len() {
        if report[i - 1] == report[i] {
            return 0;
        }
        if is_desc && report[i - 1] < report[i] {
            return 0;
        }
        if !is_desc && report[i - 1] > report[i] {
            return 0;
        }
        if report[i].abs_diff(report[i - 1]) > 3 {
            return 0;
        }
    }

    1
}

fn check_safe2(report: &str) -> usize {
    let report : Vec<usize>= report.split(" ").flat_map(|x| { x.parse::<usize>()}).collect();

    let is_desc = report[0] > report[1];

    for i in 1..report.len() {
        let mut nb_error = 0;
        if report[i - 1] == report[i] {
            nb_error += 1;
        }
        if is_desc && report[i - 1] < report[i] {
            nb_error += 1;
        }
        if !is_desc && report[i - 1] > report[i] {
            nb_error += 1;
        }
        if report[i].abs_diff(report[i - 1]) > 3 {
            nb_error += 1;
        }
    }

    1
}

pub fn part1(input: &str) -> usize {
    let mut ret = 0;
    for report in input.split("\r\n").into_iter() {
        ret += check_safe(report);
    }
    ret
}

pub fn part2(input: &str) -> usize {
    let mut ret = 0;
    for report in input.split("\r\n").into_iter() {
        ret += check_safe(report);
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2};

    #[test]
    pub fn test_part1() {
        let x = "7 6 4 2 1\r\n1 2 7 8 9\r\n9 7 6 2 1\r\n1 3 2 4 5\r\n8 6 4 4 1\r\n1 3 6 7 9";

        assert_eq!(part1(x), 2);
    }

    #[test]
    pub fn test_part1_final() {
        let input_string = include_str!("../input/2024/day2.txt");
        let result = part1(input_string);
        print!("Final value: {}\n", result);
        panic!("lol");
    }
    #[test]
    pub fn test_part2() {
        let x = "3   4\r\n4   3\r\n2   5\r\n1   3\r\n3   9\r\n3   3";
        assert_eq!(part2(x), 31);
    }

    #[test]
    pub fn test_part2_final() {
        let input_string = include_str!("../input/2024/day1.txt");
        let result = part2(input_string);
        print!("Final value: {}\n", result);
    }

}