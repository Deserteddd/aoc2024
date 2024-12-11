pub fn d11() -> (u64, u64) {
    let mut input = std::fs::read_to_string("./inputs/d11.txt").unwrap();
    let mut input = "125 17".to_string();
    let mut new = String::new();
    (0..25).for_each(|n| {
        // println!("Blink {}", n);
        let now = std::time::Instant::now();
            let mut start = 0;
            let mut end = 0;
            input.bytes().enumerate().for_each(|c| {
                if c.1 == b' ' || c.0 == input.len()-1 {
                    start = end;
                    end = c.0;
                    // println!("token at [{}..{}]: {}", start, end, &input[start..end])
                }
            });
        let elapsed = now.elapsed();
        input.clear();
        input.push_str(&new);
        new.clear();
    });
    let p1 = input.split_ascii_whitespace().count() as u64;
    (p1, 0)
}

fn update_stone(stone: &str) -> String{
    if stone == "0" {
        "1 ".to_string()
    } else if stone.len() % 2 == 0 {
        let first = &stone[..stone.len()/2];
        let second = stone[stone.len()/2..].trim_start_matches('0');
        format!("{} {} ", first, if second.is_empty() { "0 " } else { second })
    } else {
        format!("{} ", stone.parse::<u64>().unwrap()*2024)
    }
}