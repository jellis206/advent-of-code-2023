fn main() {
    // println!("{:?}", parse_text(&get_input().unwrap()));
    parse_text(&get_input().unwrap())
}

fn parse_text(input: &str) {
    let mut data = input.split("\n\n");
    let seeds: Vec<u32> = data
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| {
            if s.chars().all(|c| c.is_numeric()) {
                Some(s.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .collect();
    println!("{:?}", seeds);

    let mut maps = Vec::new();
    for m in data {
        maps.push(parse_map(m));
    }


}

fn parse_map(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| {
                    if s.chars().all(|c| c.is_numeric()) {
                        Some(s.parse::<u32>().unwrap())
                    } else {
                        None
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn get_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("./input-test1.txt")
}
