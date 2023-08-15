mod n_queens;
use n_queens::solve_n_queens;

mod word_search;
use word_search::exist;

fn main() {
    // n queens
    let n = 8;
    let queens = solve_n_queens(n);

    for q in &queens {
        println!("{}", q);
        println!();
    }

    // word search
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = String::from("ABCCED");

    if exist(board, word) {
        println!("Word exists in the board.");
    } else {
        println!("Word does not exist in the board.");
    }
}
