// Day 4
pub fn d4() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d4.txt").unwrap();
    let mut total = 0;
    let mut part2 = 0;
    let fward = ['X' as u8, 'M' as u8, 'A' as u8, 'S' as u8];
    let bward = ['S' as u8, 'A' as u8, 'M' as u8, 'X' as u8];
    let line_len = input.lines().nth(0).unwrap().chars().count();
    let line_count = input.lines().count();
    let input_bytes = input.as_bytes();
    input.lines()
        .enumerate()
        .for_each(|(line_idx, line)| {
            line.char_indices()
                .for_each(|(char_idx, chr)| {
                    // Check horizontal
                    if char_idx < line_len - 3 {
                        let w = line.get(char_idx..char_idx+4);
                        if w == Some("XMAS") || w == Some("SAMX") { total += 1 }
                    }

                    // Check vertical
                    if line_idx < line_count - 3 {
                        let column = [
                            chr as u8,
                            input_bytes[line_len*(line_idx+1) + 2*line_idx + 2 + char_idx],
                            input_bytes[line_len*(line_idx+2) + 2*line_idx + 4 + char_idx],
                            input_bytes[line_len*(line_idx+3) + 2*line_idx + 6 + char_idx],
                        ];

                        if column == fward || column == bward { total += 1 }
                    }

                    // Check diagonal
                    if char_idx < line_len - 3 && line_idx < line_count - 3 {
                        let up_down = [
                            chr as u8,
                            input_bytes[line_len*(line_idx+1) + 2*line_idx + 3 + char_idx],
                            input_bytes[line_len*(line_idx+2) + 2*line_idx + 6 + char_idx],
                            input_bytes[line_len*(line_idx+3) + 2*line_idx + 9 + char_idx],
                        ];
                        let down_up = [
                            input_bytes[line_len*(line_idx+3) + 2*line_idx + 6 + char_idx],
                            input_bytes[line_len*(line_idx+2) + 2*line_idx + 5 + char_idx],
                            input_bytes[line_len*(line_idx+1) + 2*line_idx + 4 + char_idx],
                            input_bytes[line_len*(line_idx  ) + 2*line_idx + 3 + char_idx],
                        ];

                        if up_down == fward || up_down == bward { total += 1 }
                        if down_up == fward || down_up == bward { total += 1 }
                    }

                    // Check part2
                    if char_idx < line_len - 2 && line_idx < line_count - 2 {
                        let mut valid = true;
                        let valids = ['S' as u8, 'M' as u8];
                        let mid   = input_bytes[line_len*(line_idx+1) + 2*line_idx + 3 + char_idx];
                        let top_r = input_bytes[line_len*(line_idx  ) + 2*line_idx + 2 + char_idx];
                        let bot_l = input_bytes[line_len*(line_idx+2) + 2*line_idx + 4 + char_idx];
                        let bot_r = input_bytes[line_len*(line_idx+2) + 2*line_idx + 6 + char_idx];
                        if !valids.contains(&top_r) || !valids.contains(&bot_l) || 
                           !valids.contains(&bot_r) || !valids.contains(&(chr as u8)) {
                            valid = false
                        }
                        if mid != 'A' as u8 {valid = false};
                        if chr == 'M' && bot_r != 'S' as u8 {valid = false};
                        if chr == 'S' && bot_r != 'M' as u8 {valid = false};
                        if bot_l == 'S' as u8 && top_r != 'M' as u8 {valid = false};
                        if bot_l == 'M' as u8 && top_r != 'S' as u8 {valid = false};

                        if valid {
                            part2 += 1
                        }
                    }
                }
            );
        }
    );

    (total, part2)
}