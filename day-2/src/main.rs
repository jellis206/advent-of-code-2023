fn main() {
    cube_conundrum()
}

fn cube_conundrum() {
    let game_config = std::collections::HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let game_input = get_input().unwrap();
    let mut sum = 0;
    let mut pow_sum = 0;
    game_input.lines().for_each(|line| {
        let mut is_possible = true;
        let line = line.to_ascii_lowercase();
        let start = line.find("game").unwrap();
        let end = line.find(':').unwrap();
        let game_id = line[start..end].trim();
        let id_value: i32 = game_id.split_whitespace().last().unwrap().parse().unwrap();
        let _line = line[end + 1..].trim();
        let data = _line.split(';').collect::<Vec<&str>>();

        let mut min_color_counts =
            std::collections::HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for pull in data {
            let colors_and_values = pull.split(',').collect::<Vec<&str>>();
            for c_and_v in colors_and_values {
                let mut c_and_v_sep = c_and_v.split_whitespace();
                let color_count: i32 = c_and_v_sep.next().unwrap().parse().unwrap();
                let color_name = c_and_v_sep.next().unwrap();
                if game_config.contains_key(color_name) {
                    let game_value = game_config.get(color_name).unwrap();
                    if color_count > *game_value {
                        is_possible = false;
                    }
                }

                if min_color_counts.contains_key(color_name) {
                    let current_count = min_color_counts.get(color_name).unwrap();
                    if color_count > *current_count || *current_count == 0 {
                        min_color_counts.insert(color_name, color_count);
                    }
                }
            }
        }
        println!("{}", _line);
        let mut game_sum = 0;
        for (color_name, min_count) in min_color_counts.iter() {
            println!("{}: {}", color_name, min_count);
            if *min_count > 0 {
                if game_sum == 0 {
                    game_sum = 1;
                }
                game_sum *= min_count;
            }
        }
        pow_sum += game_sum;

        if is_possible {
            sum += id_value;
        }
    });
    println!("Sum: {}", sum);
    println!("Pow sum: {}", pow_sum);
}

#[tokio::main]
async fn get_input() -> Result<String, reqwest::Error> {
    // URL of the API endpoint
    let url = "https://adventofcode.com/2023/day/2/input";
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
        .header("Referer", "https://adventofcode.com/2023/day/2")
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
