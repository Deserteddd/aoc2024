mod solutions;
use solutions::*;

fn main() {
    print_runtimes(&[
        print_day(1, d1),
        print_day(2, d2),
        print_day(3, d3),
        print_day(4, d4),
        print_day(5, d5),
        print_day(6, d6),
        print_day(7, d7),
        print_day(8, d8),
        print_day(9, d9),
        print_day(10, d10),
        print_day(11, d11)
    ]);
}

fn print_day(d: i32, func: fn() -> (u64, u64)) -> (i32, std::time::Duration) {
    println!("Day {d}");
    let now = std::time::Instant::now();
    let results = func();
    let elaplsed = now.elapsed();
    println!("  - part 1: {}", results.0);
    println!("  - part 2: {}", results.1);
    println!();
    (d, elaplsed)
}

fn print_runtimes(times: &[(i32, std::time::Duration)]) {
    println!("\nRuntimes:");
    times.iter().for_each(|(day, runtime)| 
        println!("Day {}: {:?}", day, runtime)
    );
}



