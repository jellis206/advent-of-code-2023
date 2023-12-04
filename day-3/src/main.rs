fn main() {
    engine_repair()
}

fn engine_repair() {
    let input = get_input().unwrap();
    let engine_schematic = parse_input(input);
    let (part_nums, gear_ratios) = parse_engine_schematic(&engine_schematic);
    println!("Sum of part numbers: {:?}", part_nums.iter().sum::<u32>());
    println!(
        "Product of gear ratios: {:?}",
        gear_ratios.iter().sum::<u32>()
    );
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let mut engine_schematic: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        let mut line_chars: Vec<char> = Vec::new();
        for c in line.chars() {
            line_chars.push(c);
        }
        engine_schematic.push(line_chars);
    }
    engine_schematic
}

fn parse_engine_schematic(engine_schematic: &Vec<Vec<char>>) -> (Vec<u32>, Vec<u32>) {
    let mut part_numbers: Vec<u32> = Vec::new();
    let mut gears = get_gears(engine_schematic);
    let mut gear_key: String = String::new();
    let mut is_maybe_a_gear = false;
    for i in 0..engine_schematic.len() {
        let mut part_num: String = String::new();
        let mut is_a_part = false;
        for j in 0..engine_schematic[i].len() {
            if engine_schematic[i][j].is_numeric() {
                part_num.push(engine_schematic[i][j]);
            }

            if !part_num.is_empty() && engine_schematic[i][j].is_numeric() {
                if i > 0 && !is_a_part {
                    (is_a_part, is_maybe_a_gear) = check_symbol(engine_schematic[i - 1][j]);
                    if is_maybe_a_gear {
                        gear_key = format!("{}{}", i - 1, j);
                    }
                }
                if i < engine_schematic.len() - 1 && !is_a_part {
                    (is_a_part, is_maybe_a_gear) = check_symbol(engine_schematic[i + 1][j]);
                    if is_maybe_a_gear {
                        gear_key = format!("{}{}", i + 1, j);
                    }
                }
                if j > 0 && !is_a_part {
                    (is_a_part, is_maybe_a_gear) = check_symbol(engine_schematic[i][j - 1]);
                    if is_maybe_a_gear {
                        gear_key = format!("{}{}", i, j - 1);
                    }
                }
                if j < engine_schematic[i].len() - 1 && !is_a_part {
                    (is_a_part, is_maybe_a_gear) = check_symbol(engine_schematic[i][j + 1]);
                    if is_maybe_a_gear {
                        gear_key = format!("{}{}", i, j + 1);
                    }
                }
                if i > 0 && j > 0 && !is_a_part {
                    (is_a_part, is_maybe_a_gear) = check_symbol(engine_schematic[i - 1][j - 1]);
                    if is_maybe_a_gear {
                        gear_key = format!("{}{}", i - 1, j - 1);
                    }
                }
                if i > 0 && j < engine_schematic[i].len() - 1 && !is_a_part {
                    (is_a_part, is_maybe_a_gear) = check_symbol(engine_schematic[i - 1][j + 1]);
                    if is_maybe_a_gear {
                        gear_key = format!("{}{}", i - 1, j + 1);
                    }
                }
                if i < engine_schematic.len() - 1 && j < engine_schematic[i].len() - 1 && !is_a_part
                {
                    (is_a_part, is_maybe_a_gear) = check_symbol(engine_schematic[i + 1][j + 1]);
                    if is_maybe_a_gear {
                        gear_key = format!("{}{}", i + 1, j + 1);
                    }
                }
                if i < engine_schematic.len() - 1 && j > 0 && !is_a_part {
                    (is_a_part, is_maybe_a_gear) = check_symbol(engine_schematic[i + 1][j - 1]);
                    if is_maybe_a_gear {
                        gear_key = format!("{}{}", i + 1, j - 1);
                    }
                }
            }

            if !engine_schematic[i][j].is_numeric() || j == engine_schematic[i].len() - 1 {
                if is_a_part {
                    part_numbers.push(part_num.parse::<u32>().unwrap());
                    is_a_part = false;
                }
                if is_maybe_a_gear {
                    gears
                        .get_mut(&gear_key)
                        .unwrap()
                        .push(part_num.parse::<u32>().unwrap());
                    is_maybe_a_gear = false;
                }
                part_num = String::new();
            }
        }
    }

    let mut gear_ratios: Vec<u32> = Vec::new();
    for (_, gear) in gears.iter() {
        if gear.len() == 2 {
            gear_ratios.push(gear[0] * gear[1]);
        }
    }

    (part_numbers, gear_ratios)
}

fn check_symbol(c: char) -> (bool, bool) {
    (c != '.' && !c.is_ascii_alphanumeric(), c == '*')
}

fn get_gears(engine_schematic: &[Vec<char>]) -> std::collections::HashMap<String, Vec<u32>> {
    let mut gears: std::collections::HashMap<String, Vec<u32>> = std::collections::HashMap::new();

    for (i, line) in engine_schematic.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '*' {
                gears.insert(format!("{}{}", i, j), Vec::new());
            }
        }
    }
    gears
}

#[tokio::main]
async fn get_input() -> Result<String, reqwest::Error> {
    // URL of the API endpoint
    let url = "https://adventofcode.com/2023/day/3/input";
    let client = reqwest::Client::new();
    let cookies = std::fs::read_to_string("../cookies.txt").unwrap();

    // Make a GET request
    let response = client
        .get(url)
        .header("Accept", "text/html")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Cookie", cookies.trim().to_string())
        .header("Cache-Control", "no-cache")
        .header("Pragma", "no-cache")
        .header("Referer", "https://adventofcode.com/2023/day/3")
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Read the response as text
        let text = response.text().await?;

        Ok(text)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}
