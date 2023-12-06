use std::collections::{HashMap, HashSet};

fn result(data: &str) -> (u32, u32) {
    let mut worth = 0;
    let mut pile = HashMap::<usize, (u32, u32)>::new();

    // we don't care about the card number value (here we 'enumerate' it)
    for (card, winners, numbers) in data.lines().enumerate().filter_map(|(c, l)| {
        l.split_once(':')
            .map(|(_, x)| {
                x.split_once('|').map(|(w, n)| {
                    (
                        c,
                        w.split_whitespace().collect::<HashSet<_>>(),
                        n.split_whitespace().collect::<Vec<_>>(),
                    )
                })
            })
            .unwrap()
    }) {
        let mut wins = 0;
        for n in &numbers {
            if winners.get(n).is_some() {
                wins += 1;
            }
        }

        // for part 2
        // set the number of wins for the (original) scratchcard; quantity may be
        // non-zero (from previous iterations); we do init & update in one loop
        let mut qty = 1;
        if let Some((ref mut w, ref mut q)) = pile.get_mut(&card) {
            *w = wins;
            *q += 1;
            qty = *q;
        } else {
            pile.insert(card, (wins, qty));
        }

        if wins > 0 {
            // for part 1: 2°+2¹+2²+…
            worth += 2u32.pow(wins - 1);

            // for part 2: this card *and this copies* provides aditional ones
            // so we must multiply by the amount (quantity) of copies
            for n in 1usize..=wins as usize {
                // there may be some card copies from previous iterations
                if let Some((_, ref mut q)) = pile.get_mut(&(card + n)) {
                    *q += qty;
                } else {
                    // insert new card, whose 'wins' are unknown ATM
                    pile.insert(card + n, (0, qty));
                }
            }
        }
    }
    (worth, pile.into_values().map(|(_, q)| q).sum())
}

pub fn main() {
    let (p1, p2) = result(&get_input().unwrap());
    println!("Answer day 4.1 = {p1}");
    println!("Answer day 4.2 = {p2}");
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
