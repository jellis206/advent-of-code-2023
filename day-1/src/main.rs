fn main() {
    let text = get_input().unwrap();
    let numbers: Vec<(Option<i32>, Option<i32>)> = text.lines().map(find_num).collect();
    let mut sum = 0;

    for (first_num, last_num) in numbers {
        match (first_num, last_num) {
            (Some(first_num), Some(last_num)) => {
                let combo = format!("{}{}", first_num, last_num);
                sum += combo.parse::<i32>().unwrap();
            }
            _ => println!("No numbers found"),
        }
    }
    println!("Sum: {}", sum);
}

fn find_num(line: &str) -> (Option<i32>, Option<i32>) {
    let spelled_nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let sub_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut nums: Vec<i32> = Vec::new();
    let mut new_line = String::new();

    'outer: for c in line.chars() {
        if c.is_numeric() {
            nums.push(c.to_digit(10).unwrap() as i32);
            break;
        }
        new_line.push(c);
        for (spelled_num, sub_num) in spelled_nums.iter().zip(sub_nums.iter()) {
            if new_line.contains(spelled_num) {
                nums.push(*sub_num);
                break 'outer;
            }
        }
    }

    let mut new_line = String::new();
    'outer: for c in line.chars().rev() {
        if c.is_numeric() {
            nums.push(c.to_digit(10).unwrap() as i32);
            break;
        }
        new_line.push(c);
        for (spelled_num, sub_num) in spelled_nums.iter().zip(sub_nums.iter()) {
            let reversed_spelled_num: String = spelled_num.chars().rev().collect();
            if new_line.contains(&reversed_spelled_num) {
                nums.push(*sub_num);
                break 'outer;
            }
        }
    }

    (nums.first().cloned(), nums.last().cloned())
}

#[tokio::main]
async fn get_input() -> Result<String, reqwest::Error> {
    // URL of the API endpoint
    let url = "https://adventofcode.com/2023/day/1/input";
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
        .header("Referer", "https://adventofcode.com/2023/day/1")
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
