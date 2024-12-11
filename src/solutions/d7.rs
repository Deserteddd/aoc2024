// Day 7
pub fn d7() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d7.txt").unwrap();
    let mut total = 0;
    input.lines()
        .map(|line| Equation::from_str(line))
        .for_each(|mut equation| {
            if equation.solvable() {
                total += equation.result
            }
        });
    (total, 0)
}
#[derive(Debug)]
struct Equation {
    result: u64,
    values: String
}

impl Equation {
    fn from_str(s: &str) -> Equation {
        let split_idx = s.chars().position(|c| c == ':').unwrap();
        let (r, v) = s.split_at(split_idx);
        let vals = v.get(2..)
            .unwrap()
            .to_string();

        Equation {
            result: r.parse().unwrap(),
            values: vals
        }
    }

    fn solvable(&mut self) -> bool {
        let operator_indices = self.values
            .char_indices()
            .filter(|c| c.1.is_whitespace())
            .map(|c| c.0)
            .collect::<Vec<usize>>();

        let len = operator_indices.len();

        for comb in generate_combinations(len).iter() {
            for (idx, op_idx) in operator_indices.iter().enumerate() {
                unsafe {
                    self.values.as_bytes_mut()[*op_idx] = comb.as_bytes()[idx] as u8;
                }
            }
            if self.is_ok() {
                return true
            }
        }
        false
    }

    fn is_ok(&self) -> bool {
        let mut result = 0;
        let mut current_op = '+';
        let mut current_num = String::new();

        for c in self.values.chars() {
            if c.is_numeric() {
            current_num.push(c);
            } else if c == '+' || c == '*' {
            let num = current_num.parse::<u64>().unwrap();
            if current_op == '+' {
                result += num;
            } else if current_op == '*' {
                result *= num;
            }
            current_op = c;
            current_num.clear();
            }
        }

        if !current_num.is_empty() {
            let num = current_num.parse::<u64>().unwrap();
            if current_op == '+' {
            result += num;
            } else if current_op == '*' {
            result *= num;
            }
        }

        result == self.result
    }
}

fn generate_combinations(n: usize) -> Vec<String> {
    let mut results = Vec::new();
    let mut current = vec![' '; n];
    backtrack(&mut current, 0, &mut results);
    results
}

fn backtrack(current: &mut Vec<char>, pos: usize, results: &mut Vec<String>) {
    if pos == current.len() {
        results.push(current.iter().collect());
        return;
    }

    for &c in &['*', '+'] {
        current[pos] = c;
        backtrack(current, pos + 1, results);
    }
}