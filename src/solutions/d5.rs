pub fn d5() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d5.txt").unwrap();
    let rules = input.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split('|'))
        .map(|mut rule| (
            rule.next().unwrap().parse::<i32>().unwrap(),
            rule.next().unwrap().parse::<i32>().unwrap()
        ))
        .collect::<Vec<(i32, i32)>>();

    let updates = input.lines()
        .skip(rules.len()+1)
        .map(|line| line.split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .collect::<Vec<Vec<i32>>>();

    let mut total = 0;
    let mut fixed = 0;
    for mut update in updates {
        if check_update(&mut update, &rules) {
            total += update[update.len()/2]
        } else {
            while !check_update(&mut update, &rules) {}
            fixed += update[update.len()/2]
        }
    }
    (total as u64, fixed as u64)
}


fn check_update(update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) -> bool{
    let mut complies = true;
    rules.iter().for_each(|rule| {
        if !check_rule(*rule, update) {
            complies = false;
            reorder(*rule, update);
        } 
    });
    complies
}

fn reorder(rule: (i32, i32), update: &mut Vec<i32>) {
    let first = update.iter().position(|elem| *elem == rule.0);
    let second = update.iter().position(|elem| *elem == rule.1);
    if first.is_none() || second.is_none() {return}
    update.swap(first.unwrap(), second.unwrap());
}

fn check_rule(rule: (i32, i32), update: &Vec<i32>) -> bool {
    let first = update.iter().position(|elem| *elem == rule.0);
    let second = update.iter().position(|elem| *elem == rule.1);
    if first.is_none() || second.is_none() { 
        return true;
    }
    first.unwrap() < second.unwrap()
}