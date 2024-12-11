// Day 3
pub fn d3() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d3.txt").unwrap();
    let mut total = 0;
    let mut current = String::new();
    for i in input.chars() {
        current.push(i);
        let result = validate(&current);
        match result {
            Validity::Invalid => current.clear(),
            Validity::Complete(n) => {
                total += n as u64;
                current.clear();
            },
            Validity::Partial => {}
        }
    }

    (total, 0)
}

#[derive(Debug)]
enum Validity {
    Complete(i32),
    Partial,
    Invalid
}

fn validate(ins: &str) -> Validity {
    let valid_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '(', ')', ',', 'm', 'u', 'l'];
    for i in ins.chars() {
        if !valid_chars.contains(&i) {
            return Validity::Invalid; // Contains invalid charecters
        }
    }

    let len = ins.len();
    if len < 4 {
        if ins[..len] == "mul"[..len] {
            return Validity::Partial
        }
        return Validity::Invalid // Invalid instruction
    } 
    if len == 4 {
        if ins.chars().last().unwrap() == '(' {
            return Validity::Partial
        }
        return Validity::Invalid // '(' Doesn't follow "mul"
    }

    let operand_1;
    if let Some(comma_idx) = ins.find(',') {
        if let Ok(first_num) = ins[4..comma_idx].parse::<i32>() {
            operand_1 = first_num;
        } else {
            return Validity::Invalid // First operand is complete but it's invalid
        }
        if comma_idx == len-1 {
            return Validity::Partial
        }
        if ins.chars().last().unwrap() == ')' {
            if let Ok(operand_2) = ins[comma_idx+1..len-1].parse::<i32>() {
                return Validity::Complete(operand_1 * operand_2);
            } else {
                return Validity::Invalid // Second operand is invalid
            }
        } else {
            return Validity::Partial
        }
    } else if ins[4..].parse::<i32>().is_ok() {
        return Validity::Partial
    } else {
        return Validity::Invalid // First operand is invalid
    }
}
