fn is_safe(board: &Vec<Vec<bool>>, row: usize, col: usize, n: usize) -> bool {
    for i in 0..col {
        if board[row][i] {
            return false;
        }
    }

    for i in (0..row).rev() {
        if board[i][col] {
            return false;
        }
    }

    let mut i = row as i32 - 1;
    let mut j = col as i32 - 1;
    while i >= 0 && j >= 0 {
        if board[i as usize][j as usize] {
            return false;
        }
        i -= 1;
        j -= 1;
    }

    i = row as i32 - 1;
    j = col as i32 + 1;
    while i >= 0 && j < n as i32 {
        if board[i as usize][j as usize] {
            return false;
        }
        i -= 1;
        j += 1;
    }

    true
}

fn solve_n_queens_util(board: &mut Vec<Vec<bool>>, col: usize, n: usize) -> bool {
    if col >= n {
        return true;
    }

    for i in 0..n {
        if is_safe(board, i, col, n) {
            board[i][col] = true;
            if solve_n_queens_util(board, col + 1, n) {
                return true;
            }
            board[i][col] = false;
        }
    }

    false
}

pub fn solve_n_queens(n: usize) -> Vec<String> {
    let mut board = vec![vec![false; n]; n];
    let mut queens = Vec::new();

    if solve_n_queens_util(&mut board, 0, n) {
        for row in &board {
            queens.push(
                row.iter()
                    .map(|&queen| {
                        if queen {
                            "Q".to_string()
                        } else {
                            "X".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(""),
            );
        }
    }

    queens
}
