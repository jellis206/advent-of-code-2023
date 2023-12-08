fn main() {
    println!("{:?}", get_seeds(&get_input().unwrap()));
}

fn get_seeds(input: &str) -> Vec<String> {
    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| {
            if s.chars().all(|c| c.is_numeric()) {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn get_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("/input-test1.txt")
}

// #[tokio::main]
// async fn get_input() -> Result<String, reqwest::Error> {
//     // URL of the API endpoint
//     let url = "https://adventofcode.com/2023/day/5/input";
//     let client = reqwest::Client::new();
//     let cookies = std::fs::read_to_string("../cookies.txt").unwrap();
//
//     // Make a GET request
//     let response = client
//         .get(url)
//         .header("Accept", "text/html")
//         .header("Accept-Language", "en-US,en;q=0.9")
//         .header("Cookie", cookies.trim().to_string())
//         .header("Cache-Control", "no-cache")
//         .header("Pragma", "no-cache")
//         .header("Referer", "https://adventofcode.com/2023/day/5")
//         .send()
//         .await?;
//
//     // Check if the request was successful
//     if response.status().is_success() {
//         // Read the response as text
//         let text = response.text().await?;
//
//         Ok(text)
//     } else {
//         Err(response.error_for_status().unwrap_err())
//     }
// }
