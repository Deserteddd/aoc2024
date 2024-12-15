// 92092395920 is too low
pub fn d9() -> (u64, u64) {

    let input = std::fs::read_to_string("./inputs/d9.txt").unwrap();
    // let input = "2333133121414131402";
    // let input = "12345";
    let decoded = decode(&input);
    // print_disk(&decoded);
    let part_1 = part_1(decoded.clone());
    let part_2 = part_2(decoded);

    (part_1, part_2) // too low
}
fn print_disk(disk: &Vec<Option<u32>>) {
    disk
        .iter()
        .for_each(|elem| 
            if let Some(n) = elem {print!("{n}")} 
            else {print!(".")
        });

    println!()
}
fn part_2(mut disk: Vec<Option<u32>>) -> u64 {
    let mut is_start = true;
    let mut start = disk
        .iter()
        .enumerate()
        .find(|(_, c)| c.is_some())
        .unwrap();
    let mut blocks: Vec<(usize, usize)> = vec![];
    for i in start.0..disk.len() {
        if disk[i] == *start.1 {
            if is_start {
                is_start = false;
            }
        } else {
            is_start = true;
            if disk.get(start.0..i).unwrap()[0] != None {
                blocks.push((start.0, i));
            }
            start = (i, &disk[i]);
        }
    }
    if start.1.is_some() {
        blocks.push((start.0, disk.len()));
    }
    // println!("Blocks: {:?}", blocks);
    blocks.iter().rev().for_each(|block| {
        let block_len = block.1-block.0;
        // print_disk(&disk);
        if let Some(empty_start) = find_empty_block(&disk, block_len) {
            if empty_start < block.0 {
            // println!("{:?}", disk.get(block.0..block.1));
            for i in 0..block_len {
                disk[empty_start+i] = disk[block.0+i].take()
            }
            }
        }

    });
    calculate_checksum(&disk)
}

fn find_empty_block(disk: &Vec<Option<u32>>, size: usize) -> Option<usize> {
    let mut count = 0;
    let mut is_start = true;
    let mut start = disk
        .iter()
        .enumerate()
        .find(|(_, c)| c.is_none())
        .unwrap().0;

    for i in start..disk.len() {
        if disk[i].is_none() {
            if is_start {
                is_start = false;
                start = i
            }
            count += 1;
            if count == size {
                return Some(start)
            }
        } else {
            count = 0;
            is_start = true
        }
    }
    None
}

fn part_1(mut input: Vec<Option<u32>>) -> u64 {
    let mut empty_spaces = input
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, c)| c.is_none())
        .map(|(idx, _)| idx);

    let mut rev_idx = input.len()-1;
    while let Some(empty_idx) = empty_spaces.next() {
        while input[rev_idx].is_none() && empty_idx < rev_idx {
            rev_idx -= 1;
        }
        input[empty_idx] = input[rev_idx].take()
    }
    calculate_checksum(&input)
}

fn decode(input: &str) -> Vec<Option<u32>> {
    let mut file = Vec::new();
    let mut block_start = true;
    let mut id: u32 = 0;
    input.chars().for_each(|c| {
        let num = c as u8 - b'0';
        for _ in 0..num {
            match block_start {
                true => file.push(Some(id)),
                false => file.push(None),
            }
        }
        if block_start {id += 1};
        block_start ^= true;
    });
    file
}

fn calculate_checksum(disk: &Vec<Option<u32>>) -> u64 {
    disk
        .iter()
        .enumerate()
        .filter(|elem| elem.1.is_some())
        .map(|(idx, val)| idx as u64 * val.unwrap() as u64)
        .sum()
}