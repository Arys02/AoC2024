use std::collections::binary_heap::BinaryHeap;
use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut bheap1: BinaryHeap<usize> = BinaryHeap::new();
    let mut bheap2: BinaryHeap<usize> = BinaryHeap::new();
    for line in input.split("\r\n").into_iter() {
        let x : Vec<&str>= line.split("   ").collect();
        let v1 = x[0].parse::<usize>().unwrap();
        let v2 = x[1].parse::<usize>().unwrap();
        bheap1.push(v1);
        bheap2.push(v2);
    }

    let mut ret = 0;
    loop {
        let v1 = bheap1.pop();
        if v1.is_none() {
            break
        }
        let v2 = bheap2.pop();
        ret += v1.unwrap().abs_diff(v2.unwrap());
    }
    ret
}

pub fn part2(input: &str) -> usize {
    let mut hmap = HashMap::new();

    for line in input.split("\r\n").into_iter() {
        let x : Vec<&str>= line.split("   ").collect();

        hmap.entry(x[0]).or_insert((0, 0)).0 += 1;
        hmap.entry(x[1]).or_insert((0, 0)).1 += 1;
    }

    let mut ret = 0;
    for (k, v) in hmap.iter() {
        ret += k.parse::<usize>().unwrap() * v.1 * v.0;
    }

    ret
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    pub fn test_part1() {
        let x = "3   4\r\n4   3\r\n2   5\r\n1   3\r\n3   9\r\n3   3";


        assert_eq!(part1(x), 11);
    }

    #[test]
    pub fn test_part1_final() {
        let input_string = include_str!("../input/2024/day1.txt");
        let result = part1(input_string);
        print!("Final value: {}\n", result);
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