pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        "player X won".to_string()
    } else if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        "player O won".to_string()
    } else {
        "tie".to_string()
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let main = (0..3).all(|i| table[i][i] == player);
    let anti = (0..3).all(|i| table[i][2 - i] == player);
    main || anti
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    table.iter().any(|row| row.iter().all(|&c| c == player))
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (0..3).any(|j| (0..3).all(|i| table[i][j] == player))
}