fn main() {
    scratchers();
}

fn scratchers() {
    let input = get_input().unwrap();
    println!("Points: {}", get_points(&input));
    println!("Card Count: {}", get_card_count(&input));
}

fn get_card_count(input: &str) -> u32 {
    let updated_cards: Vec<&str> = input.lines().collect();
    let mut card_counts: std::collections::HashMap<usize, u32> =
        input.lines().enumerate().map(|(i, _)| (i, 1)).collect();
    let mut total_cards: u32 = 0;

    for i in 0..updated_cards.len() {
        for _j in 1..=*card_counts.get(&i).unwrap() {
            let (card_nums, winning_nums) = get_nums(updated_cards[i]);
            let wins = card_nums
                .iter()
                .filter(|x| winning_nums.contains(x))
                .count();
            for k in 1..=wins {
                if i + k >= updated_cards.len() {
                    break;
                }
                let index = i + k;
                let new_count = card_counts.get(&index).unwrap() + 1;
                card_counts.insert(i + k, new_count);
            }
        }
        total_cards += card_counts.get(&i).unwrap();
    }
    total_cards
}

fn get_points(input: &str) -> u32 {
    let mut points: u32 = 0;
    input.lines().for_each(|line| {
        let (card_nums, winning_nums) = get_nums(line);
        let mut card_points = 0;
        for c_num in card_nums.iter() {
            if winning_nums.contains(c_num) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }
        points += card_points;
    });

    points
}

fn get_nums(line: &str) -> (Vec<&str>, Vec<&str>) {
    let mut separated_nums = line.split('|');
    let card_nums = separated_nums
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|x| x.chars().all(|c| c.is_numeric()))
        .collect::<Vec<&str>>();
    let winning_nums = separated_nums
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|x| x.chars().all(|c| c.is_numeric()))
        .collect::<Vec<&str>>();

    (card_nums, winning_nums)
}

#[tokio::main]
async fn get_input() -> Result<String, reqwest::Error> {
    // URL of the API endpoint
    let url = "https://adventofcode.com/2023/day/4/input";
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
        .header("Referer", "https://adventofcode.com/2023/day/4")
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
