
// EJERCICIO 1.1

// cuenta la cantidad de subsets en nums que suman k con backtracking
pub fn subset_sum(nums: Vec<i32>, k: i32) -> i32 {
    fn subset_sum_helper(nums: &Vec<i32>, i: usize, k: i32) -> i32 {

        if k == 0 { 1 } // con este caso recorto mas el arbol

        else if i == 0 {
            if k == nums[i] { 1 }
            else { 0 }
        }
        else{
            subset_sum_helper(nums, i-1, k) +
                subset_sum_helper(nums, i-1, k-nums[i])
        }
    }
    subset_sum_helper(&nums, nums.len()-1, k)
}

// EJERCICIO 1.2

fn is_magic(square: &Vec<Vec<u8>>) -> bool {
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

// busco cuadrados magicos con brute force
pub fn magic_squares(n: u8){

    fn find_all_squares(square: Vec<Vec<u8>>, nums: Vec<u8>, pos: (usize, usize)) -> Vec<Vec<Vec<u8>>> {

        if nums.is_empty() {
            let mut result: Vec<Vec<Vec<u8>>> = Vec::new();
            result.push(square.clone());
            result

        } else {
            let mut results: Vec<Vec<Vec<u8>>> = Vec::new();
            for i in 0..nums.len() {
                let mut new_square = square.clone();
                new_square[pos.0][pos.1] = nums[i]; // square with new num

                let mut new_nums = nums.clone();
                new_nums.remove(i); // erase inserted num

                let new_pos = // new position to do recursion
                    if pos.1 == square.len() - 1 {
                    (pos.0 + 1, 0) } else { (pos.0, pos.1 + 1)
                };
                results.extend(find_all_squares( // recursion
                    new_square,
                    new_nums,
                    new_pos,
                ));
            }
            results
        }
    }

    let initial_square = vec![vec![0; n as usize]; n as usize];
    let initial_nums = (1..=n*n).collect::<Vec<u8>>();
    let result = find_all_squares(initial_square, initial_nums, (0, 0));

    for sq in result{
        if is_magic(&sq) { println!("{:?}", sq) }
    }
}

