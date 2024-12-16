pub fn d15() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d15.txt").unwrap();

    let mut warehouse = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().map(|ch| match ch {
            '.' => Object::Space,
            'O' => Object::Box,
            '@' => Object::Robot,
            '#' => Object::Wall,
            _   => panic!("Input contains invalid character {}", ch)
        }).collect::<Vec<Object>>())
        .collect::<Vec<Vec<Object>>>();

    let mut robot_pos = warehouse
        .iter()
        .enumerate()
        .find(|row| row.1.contains(&Object::Robot))
        .map(|line| (line.0, line.1.iter().position(|c| *c == Object::Robot).unwrap()))
        .unwrap();
    input
        .lines()
        .skip_while(|line| !line.is_empty())
        .collect::<String>()
        .chars()
        .for_each(|rule| {
            update_warehouse(&mut warehouse, &mut robot_pos, rule);
        });

    let p1 = coordinate_sum(&warehouse);
    (p1, 0)
}

fn coordinate_sum(wh: &Vec<Vec<Object>>) -> u64 {
    let mut total = 0;
    for i in 0..wh.len() {
        for j in 0..wh[0].len() {
            if wh[i][j] == Object::Box {
                total += 100 * i + j
            }
        }
    }
    total as u64
}

fn update_warehouse(wh: &mut Vec<Vec<Object>>, robot: &mut (usize, usize), rule: char) {
    match rule {
        '<' => {
            match wh[robot.0][robot.1 - 1] {
                Object::Wall => return,
                Object::Space => {
                    wh[robot.0].swap(robot.1, robot.1 - 1);
                    *robot = (robot.0, robot.1 - 1);
                },
                Object::Box => {
                    let mut free: Option<usize> = None;
                    let mut i = robot.1 - 2;
                    loop {
                        match wh[robot.0][i] {
                            Object::Wall => break,
                            Object::Box => i -= 1,
                            Object::Space => {
                                free = Some(i);
                                break
                            },
                            _ => {}
                        }
                    }
                    if let Some(start) = free {
                        for i in start..robot.1 {
                            wh[robot.0].swap(i, i+1);
                        }
                        *robot = (robot.0, robot.1 - 1);
                    }
                }
                _ => {}
            }
        },
        '>' => {
            match wh[robot.0][robot.1 + 1] {
                Object::Wall => return,
                Object::Space => {
                    wh[robot.0].swap(robot.1, robot.1 + 1);
                    *robot = (robot.0, robot.1 + 1);
                },
                Object::Box => {
                    let mut free: Option<usize> = None;
                    let mut i = robot.1 + 1;
                    loop {
                        match wh[robot.0][i] {
                            Object::Wall => break,
                            Object::Box => i += 1,
                            Object::Space => {
                                free = Some(i);
                                break
                            },
                            _ => {}
                        }
                    }
                    if let Some(end) = free {
                        for i in (robot.1..end).rev() {
                            wh[robot.0].swap(i, i+1);
                        }
                        *robot = (robot.0, robot.1 + 1);
                    }

                }
                _ => {}
            }
        },
        '^' => {
            match wh[robot.0-1][robot.1] {
                Object::Wall => return,
                Object::Space => {
                    wh[robot.0-1][robot.1] = Object::Robot;
                    wh[robot.0][robot.1] = Object::Space;
                    *robot = (robot.0-1, robot.1);
                },
                Object::Box => {
                    let mut free: Option<usize> = None;
                    let mut i = robot.0 - 1;
                    loop {
                        match wh[i][robot.1] {
                            Object::Wall => break,
                            Object::Box => i -= 1,
                            Object::Space => {
                                free = Some(i);
                                break
                            },
                            _ => {}
                        }
                    }
                    if let Some(start) = free {
                        for i in start..robot.0 {
                            wh[i][robot.1] = std::mem::replace(&mut wh[i+1][robot.1], Object::Space) 
                        }
                        *robot = (robot.0 - 1, robot.1);
                    }
                }
                _ => {}
            }
        },
        'v' => {
            match wh[robot.0+1][robot.1] {
                Object::Wall => return,
                Object::Space => {
                    wh[robot.0+1][robot.1] = Object::Robot;
                    wh[robot.0][robot.1] = Object::Space;
                    *robot = (robot.0+1, robot.1);
                },
                Object::Box => {
                    let mut free: Option<usize> = None;
                    let mut i = robot.0 + 1;
                    loop {
                        match wh[i][robot.1] {
                            Object::Wall => break,
                            Object::Box => i += 1,
                            Object::Space => {
                                free = Some(i);
                                break
                            },
                            _ => {}
                        }
                    }
                    if let Some(end) = free {
                        for i in (robot.0..end).rev() {
                            let temp = wh[i][robot.1];
                            wh[i][robot.1] = wh[i+1][robot.1];
                            wh[i+1][robot.1] = temp;
                        }
                        *robot = (robot.0 + 1, robot.1);
                    }
                }
                _ => {}
            }
        },
        _ => {}
    }
}


fn print_warehouse(wh: &Vec<Vec<Object>>) {
    wh.iter()
        .for_each(|row| {
            row.iter().for_each(|c| print!("{}", match c {
                Object::Box => 'O',
                Object::Robot => '@',
                Object::Space => '.',
                Object::Wall => '#'
            }));
            println!();
        });
    println!();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Object {
    Space,
    Box,
    Robot,
    Wall
}