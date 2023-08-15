pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let rows = board.len();
    let cols = board[0].len();
    let word_chars: Vec<char> = word.chars().collect();

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        row: isize,
        col: isize,
        index: usize,
    ) -> bool {
        if index == word.len() {
            return true;
        }

        if row < 0
            || row >= board.len() as isize
            || col < 0
            || col >= board[0].len() as isize
            || board[row as usize][col as usize] != word[index]
        {
            return false;
        }

        let original = board[row as usize][col as usize];
        board[row as usize][col as usize] = '#';

        if backtrack(board, word, row + 1, col, index + 1)
            || backtrack(board, word, row - 1, col, index + 1)
            || backtrack(board, word, row, col + 1, index + 1)
            || backtrack(board, word, row, col - 1, index + 1)
        {
            return true;
        }

        board[row as usize][col as usize] = original;
        false
    }

    for row in 0..rows {
        for col in 0..cols {
            if board[row][col] == word_chars[0]
                && backtrack(
                    &mut board.clone(),
                    &word_chars,
                    row as isize,
                    col as isize,
                    0,
                )
            {
                return true;
            }
        }
    }

    false
}
