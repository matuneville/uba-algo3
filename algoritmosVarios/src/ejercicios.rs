pub(crate) mod ejercicios_help;

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
        if ejercicios_help::is_magic(&sq) { println!("{:?}", sq) }
    }
}


pub fn magic_squares_bt(n: u8){

    let magic_num = (n*n*n + n)/2;

    fn find_magic_squares(result: &mut Vec<Vec<Vec<u8>>>, square: Vec<Vec<u8>>, nums: Vec<u8>, pos: (usize, usize), magic_num: &u8) {

        if ! ejercicios_help::check_full_lines(&square, &magic_num){
            return;
        }

        if nums.is_empty() {
            result.push(square.clone());
            return;

        } else {
            for i in 0..nums.len() {
                let mut new_square = square.clone();
                new_square[pos.0][pos.1] = nums[i]; // square with new num

                let mut new_nums = nums.clone();
                new_nums.remove(i); // erase inserted num

                let new_pos = // new position to do recursion
                    if pos.1 == square.len() - 1 {
                        (pos.0 + 1, 0) } else { (pos.0, pos.1 + 1)
                    };
                find_magic_squares( // recursion
                                    result,
                                    new_square,
                                    new_nums,
                                    new_pos,
                                    magic_num);
            }
        }
    }

    let mut result: Vec<Vec<Vec<u8>>> = vec![];
    let initial_square = vec![vec![0; n as usize]; n as usize];
    let initial_nums = (1..=n*n).collect::<Vec<u8>>();
    find_magic_squares(&mut result, initial_square, initial_nums, (0, 0), &magic_num);

    for sq in result {
        println!("{:?}", sq)
    }
}
