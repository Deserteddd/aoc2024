// Day 6
pub fn d6() -> (u64, u64) {
    let input = std::fs::read_to_string("./inputs/d6.txt").unwrap();
    let mut total = 1;
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_row = grid
        .iter()
        .position(|line| line.contains(&('^')))
        .unwrap();
    let start_col = grid[start_row]
        .iter()
        .position(|chr| *chr == '^')
        .unwrap();
        
    solve_grid(&mut grid, (start_row, start_col), Dir::U);

    grid.iter().for_each(|line|
        line.iter().for_each(|c| if *c == 'X' {total += 1})   
    );
    (total, 0)
}

pub enum Dir {L, R, U, D}
impl Dir {
    fn rotate(self) -> Self {
        match self {
            Dir::U => Dir::R,
            Dir::R => Dir::D,
            Dir::D => Dir::L,
            Dir::L => Dir::U
        }
    }
}

fn solve_grid(grid: &mut Vec<Vec<char>>, pos: (usize, usize), dir: Dir) {
    // println!();
    // grid.iter().for_each(|row| println!("{:?}", row));
    match dir {
        Dir::U => {
            if pos.0 == 0 {
                return 
            } else if grid[pos.0-1][pos.1] == '#' {
                solve_grid(grid, pos, dir.rotate());
            } else {
                grid[pos.0][pos.1] = 'X';
                solve_grid(grid, (pos.0-1, pos.1), dir);
            }
        },
        Dir::R => {
            if pos.1 == grid[pos.0].len()-1 { 
                return 
            } else if grid[pos.0][pos.1+1] == '#' {
                solve_grid(grid, pos, dir.rotate());
            } else {
                grid[pos.0][pos.1] = 'X';
                solve_grid(grid, (pos.0, pos.1+1), dir);
            }
        },
        Dir::D => {
            if pos.0 == grid.len()-1 { 
                return 
            } else if grid[pos.0+1][pos.1] == '#' {
                solve_grid(grid, pos, dir.rotate());
            } else {
                grid[pos.0][pos.1] = 'X';
                solve_grid(grid, (pos.0+1, pos.1), dir);
            }
        },
        Dir::L => {
            if pos.1 == 0 { 
                return 
            } else if grid[pos.0][pos.1-1] == '#' {
                solve_grid(grid, pos, dir.rotate());
            } else {
                grid[pos.0][pos.1] = 'X';
                solve_grid(grid, (pos.0, pos.1-1), dir);
            }
        }
    }
}