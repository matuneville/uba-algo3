mod ejercicios_greedy;
use crate::ejercicios_greedy::{gasolineras, max_k_sum, parejas};

fn main() {
    let s = vec![0,3,4,7,10,11,16,21,25,26,27,30];

    println!("{:?}", gasolineras(& s, 5));
}
