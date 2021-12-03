use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("day2.txt").expect("Something went wrong reading the file");
    let commands = contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let mut depth = 0;
    let mut horiz = 0;
    for command in &commands {
        let params = command.split_whitespace().collect::<Vec<&str>>();
        let direction = params[0];
        let value = params[1].parse::<i32>().unwrap_or(0);
        match direction {
            "forward" => horiz += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Wrong command"),
        };
    }
    print!("Result: {}", depth * horiz);
}
