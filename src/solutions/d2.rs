// Day 2
pub fn d2() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d2.txt").unwrap();
    let mut safe_count = 0;
    for line in input.lines() {
        let report = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if check_report_safety(&report) { 
            safe_count += 1;
        }
    }
    (safe_count, 0)
}

fn check_report_safety(report: &[i32]) -> bool {
    let mut is_safe = true;
    let mut previous = report[0];
    let ascending = previous < report[1];
    for num in report.iter().skip(1) {
        let diff = previous - num;
        if !(
        (diff >= -3 && diff < 0 &&  ascending) || 
        (diff <=  3 && diff > 0 && !ascending)
        ) {

            is_safe = false;
        }
        previous = *num
    }
    is_safe
}