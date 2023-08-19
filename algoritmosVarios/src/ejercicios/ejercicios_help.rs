pub fn is_magic(square: &Vec<Vec<u8>>) -> bool {
    let mut sum: u8 = 0;
    for i in 0..square.len(){
        sum += square[0][i];
    }
    for row in 0..square.len(){
        let mut sum_line = 0;
        let mut sum_vert = 0;
        for col in 0..square.len(){
            sum_line += square[row][col];
            sum_vert += square[col][row];
        }
        if sum != sum_line || sum != sum_vert{ return false; }
    }
    let mut sum_diag1 = 0;
    let mut sum_diag2 = 0;
    for diag in 0..square.len(){
        sum_diag1 += square[diag][diag];
        sum_diag2 += square[diag][square.len()-diag-1];
    }
    if sum != sum_diag1 || sum != sum_diag2 { return false; }

    return true;
}

pub fn check_full_lines(square: &Vec<Vec<u8>>, magic_num: &u8) -> bool{
    for row in 0..square.len() {
        let mut is_cero = false;
        let mut sum_line = 0;
        for col in 0..square[row].len() {
            sum_line += square[row][col];
            if square[row][col] == 0 {
                is_cero = true;
                break;
            }
        }
        if ! is_cero {
            if *magic_num != sum_line { return false; }
        }
    }
    for col in 0..square.len() {
        let mut is_cero = false;
        let mut sum_col = 0;
        for row in 0..square.len() {
            sum_col += square[row][col];
            if square[row][col] == 0 {
                is_cero = true;
                break;
            }
        }
        if ! is_cero {
            if *magic_num != sum_col { return false; }
        }
    }
    let mut sum_diag1 = 0;
    let mut sum_diag2 = 0;
    let mut is_cero_diag1 = false;
    let mut is_cero_diag2 = false;
    for i in 0..square.len() {
        sum_diag1 += square[i][i];
        sum_diag2 += square[i][square.len() - i - 1];
        if square[i][i] == 0 { is_cero_diag1 = true; }
        if square[i][square.len() - i - 1] == 0 { is_cero_diag2 = true; }
    }
    if ! is_cero_diag1 {
        if *magic_num != sum_diag1 { return false; }
    }
    if ! is_cero_diag2 {
        if *magic_num != sum_diag2 { return false; }
    }

    true
}