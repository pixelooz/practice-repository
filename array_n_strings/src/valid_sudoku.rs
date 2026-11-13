pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut row: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut col: HashMap<usize, HashSet<char>> = HashMap::new();

    let mut square: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

    for r in 0..9 as usize {
        for c in 0..9 as usize {
            if board[r][c] == '.' {
                continue;
            }
            if !row.entry(r).or_default().insert(board[r][c]) {
                return false;
            }
            if !col.entry(c).or_default().insert(board[r][c]) {
                return false;
            }
            if !square
                .entry((r / 3, c / 3))
                .or_default()
                .insert(board[r][c])
            {
                return false;
            }
        }
    }
    true
}
