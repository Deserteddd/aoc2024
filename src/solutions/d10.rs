use itertools::Itertools;

pub fn d10() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d10.txt").unwrap();
    let trailheads = input
        .lines()
        .enumerate()
        .map(|line| {
            (line.0, line.1
                .char_indices()
                .positions(|c| c.1 == '0')
                .collect::<Vec<usize>>()
            )
        })
        .collect::<Vec<(usize, Vec<usize>)>>();

    let width = input.lines().nth(0).unwrap().chars().count();
    let mut total1 = 0;
    let mut total2 = 0;
    trailheads.iter().for_each(|row| {
        row.1
            .iter()
            .for_each(|c| {
                let score = get_trailhead_score(&input, (row.0, *c), width);
                total1 += score.0 as u64;
                total2 += score.1 as u64;
            });
    });
    (total1, total2)
}

fn get_trailhead_score(input: &str, head: (usize, usize), width: usize) -> (i32, i32) {
    let mut total1 = 0;
    let mut total2 = 0;
    let mut visited = vec![];
    walk(input, width, head, &mut total1, &mut total2, &mut visited);
    (total1, total2)
}

fn walk(
    input: &str, 
    width: usize, 
    pos: (usize, usize), 
    acc1: &mut i32, 
    acc2: &mut i32, 
    visited_peaks: &mut Vec<(usize, usize)>
) {
    if height_at(input, pos, width) == 9{
        if !visited_peaks.contains(&pos) {
            visited_peaks.push(pos);
            *acc1 += 1;
        }
        *acc2 += 1;
        return;
    }
    let (u, d, l, r): (u8, u8, u8, u8) = (
        height_at(input, (pos.0-1, pos.1), width),
        height_at(input, (pos.0+1, pos.1), width),
        height_at(input, (pos.0, pos.1-1), width),
        height_at(input, (pos.0, pos.1+1), width),
    );
    if u - height_at(&input, pos, width) == 1 {
        walk(input, width, (pos.0-1, pos.1), acc1, acc2, visited_peaks);
    }
    if d - height_at(&input, pos, width) == 1 {
        walk(input, width, (pos.0+1, pos.1), acc1, acc2, visited_peaks);
    }
    if l - height_at(&input, pos, width) == 1 {
        walk(input, width, (pos.0, pos.1-1), acc1, acc2, visited_peaks);
    }
    if r - height_at(&input, pos, width) == 1 {
        walk(input, width, (pos.0, pos.1+1), acc1, acc2, visited_peaks);
    }
}

const fn height_at(s: &str, pos: (usize, usize), width: usize) -> u8 {
    let idx = width * pos.0 + 2 * pos.0 + pos.1;
    if idx >= s.len() {
        return 0
    }
    s.as_bytes()[idx] - b'0'
}


