fn check_safe(report: Vec<usize>) -> usize {
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


pub fn part1(input: &str) -> usize {
    let mut ret = 0;
    for report in input.split("\r\n").into_iter() {
        let report_v: Vec<usize> = report.split(" ").flat_map(|x| x.parse::<usize>()).collect();
        ret += check_safe(report_v);
    }
    ret
}

pub fn part2(input: &str) -> usize {
    let mut ret = 0;
    for report in input.split("\r\n").into_iter() {

        let report_v: Vec<usize> = report.split(" ").flat_map(|x| x.parse::<usize>()).collect();
        let len = report_v.len();
        let is_safe = check_safe(report_v.clone());

        if is_safe == 0 {
            for i in 0..len {
                let mut clone_report = report_v.clone();
                clone_report.remove(i);

                let is_unsafe_without_i = check_safe(clone_report);
                if is_unsafe_without_i == 1 {
                    ret += 1;
                    break;
                }

            }

        }
        else if is_safe == 1 {
            ret += 1
        }

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
        panic!("{}", result);
    }
    #[test]
    pub fn test_part2() {
        let x = "7 6 4 2 1\r\n1 2 7 8 9\r\n9 7 6 2 1\r\n1 3 2 4 5\r\n8 6 4 4 1\r\n1 3 6 7 9";
        assert_eq!(part2(x), 4);
    }

    #[test]
    #[should_panic()]
    pub fn test_part2_final() {
        let input_string = include_str!("../input/2024/day2.txt");
        let result = part2(input_string);
        panic!("{}", result);
    }
}
