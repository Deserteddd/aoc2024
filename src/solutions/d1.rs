// Day 1

pub fn d1() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d1.txt").unwrap();
    (d1p1(&input) as u64, d1p2(&input) as u64)
}
fn d1p1(input: &str) -> i32 {
    let (list_l, list_r) = split_to_lists(input);

    let mut total = 0;
    (0..input.lines().count()).for_each(|i| {
        total += (list_l[i]-list_r[i]).abs();
    });
    total
}

fn d1p2(input: &str) -> i32 {
    let (list_l, list_r) = split_to_lists(input);
    let mut total = 0;
    list_l.iter().for_each(|nl| {
        let matches = list_r.iter().filter(|nr| *nr == nl).count() as i32;
        total += nl*matches
    });
    total
}

fn split_to_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_l: Vec<i32> = Vec::with_capacity(1000);
    let mut list_r: Vec<i32> = Vec::with_capacity(1000);

    input.lines().for_each(|line| {
        let num_l: i32 = line.get(0..5).unwrap().parse().unwrap();
        let num_r: i32 = line.get(8.. ).unwrap().parse().unwrap();
        insert_sorted(&mut list_l, num_l);
        insert_sorted(&mut list_r, num_r);
    });

    (list_l, list_r)
}

fn insert_sorted(vec: &mut Vec<i32>, n: i32) {
    if vec.is_empty() {
        vec.push(n);
        return
    }

    let mut i: usize = 0;
    while i < vec.len() && vec[i] < n {
        i += 1
    }
    vec.insert(i, n);
}