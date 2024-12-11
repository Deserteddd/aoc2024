use itertools::*;

pub fn d8() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d8.txt").unwrap();
//     let input =
// "............
// ........0...
// .....0......
// .......0....
// ....0.......
// ......A.....
// ............
// ............
// ........A...
// .........A..
// ............
// ............";

    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().len();
    let mut antinodes = vec![vec![false; width]; height];
    let frequencies = input
        .chars()
        .unique()
        .filter(|c| *c != '\n' && *c != '\r' && *c != '.')
        .collect::<Vec<char>>();

    frequencies.iter().for_each(|f| {
        let mut locations: Vec<(usize, usize)>  = vec![];
        input.lines().enumerate().for_each(|(line_idx, line)| {
            if line.contains(*f) {
                line.char_indices()
                    .positions(|c| c.1 == *f)
                    .map(|idx| (line_idx, idx))
                    .for_each(|location| locations.push(location));
            }
        });
        locations.iter()
            .combinations(2)
            .for_each(|comb| {
                let row_dist = comb[0].0 as i32 - comb[1].0 as i32;
                let top_y = comb[0].0 as i32 + row_dist;
                let bot_y = comb[1].0 as i32 - row_dist;

                let col_dist = comb[0].1 as i32 - comb[1].1 as i32;
                let top_x = comb[0].1 as i32 + col_dist;
                let bot_x = comb[1].1 as i32 - col_dist;

                if top_y >= 0 && top_x >= 0 && top_x < width as i32 {
                    antinodes[top_y as usize][top_x as usize] = true
                }
                if bot_x >= 0 && bot_y < height as i32 && bot_x < width as i32 {
                    antinodes[bot_y as usize][bot_x as usize] = true
                }
            });
    });
    let mut total1 = 0;
    antinodes.iter().enumerate().for_each(|(_, n)| {
        n.iter().enumerate().for_each(|(_, c)| {
            if *c { total1 += 1 };
            // print!("{}", if *c {
            //     '#'
            // } else if let Some(c) = input.lines().nth(row).unwrap().chars().nth(col) {
            //     c
            // } else {
            //     panic!()
            // })
        });
        // println!();
    });
    (total1, 0)
}