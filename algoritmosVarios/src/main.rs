mod ejercicios_greedy;
use crate::ejercicios_greedy::{max_k_sum, parejas};

fn main() {
    let s = vec![19, 7, 5, 6, 1];

    println!("{:?}", max_k_sum(s.clone(), 2));
}
