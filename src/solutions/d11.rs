use cached::proc_macro::cached;

pub fn d11() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d11.txt").unwrap();
    // let input = "125 17".to_string();
    let mut p1 = 0;
    let mut p2 = 0;
    let stones = input
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect::<Vec<u64>>();

    for stone in stones.iter().enumerate() {
        p1 += blink(*stone.1, 0, 25);
        p2 += blink(*stone.1, 0, 75);
    }
    (p1, p2)
}

#[cached]
fn blink(stone: u64, iter: u8, max_iter: u8) -> u64 {
    if iter == max_iter {
        return 1;
    }
    if stone == 0 {
        return blink(1, iter + 1, max_iter);
    }
    let digits = stone.to_string();
    let digit_count = digits.len();
    if digit_count % 2 == 0 {
        let mid = digit_count / 2;
        let left = digits.get(..mid).unwrap().parse().unwrap();
        let right = digits.get(mid..).unwrap().parse().unwrap();
        return blink(left, iter + 1, max_iter) + blink(right, iter + 1, max_iter);
    } else {
        return blink(stone * 2024, iter + 1, max_iter)
    }
}

