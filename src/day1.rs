use std::fs;

// https://adventofcode.com/2021/day/1
pub fn solve() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let nums = contents
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    let mut count: i32 = 0;
    for (pos, i) in nums.iter().enumerate() {
        if pos > 0 && i - nums[pos - 1] > 0 {
            count += 1
        }
    }

    // part1
    println!("Part 2 answer: {}", count);
    count = 0;
    let mut sums = Vec::new();
    for window in nums.windows(3) {
        let sum = window.iter().fold(0, |acc, i| i + acc);
        match sums.last() {
            None => sums.push(sum),
            Some(inner) => {
                if *inner < sum {
                    count += 1
                }
                sums.push(sum)
            }
        }
    }

    // part2
    println!("Part 2 answer: {}", count);
}
