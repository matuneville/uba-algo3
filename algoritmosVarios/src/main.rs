mod ejercicios_dp;
use crate::ejercicios_dp::{tragabilletes_topdown, tragabilletes_bt, astro_void, astro_dp_topdown, astro_dp_bottomup};

fn main() {
    let asteroides = vec![3, 2, 5, 6];

    let res = astro_dp_bottomup(& asteroides);

    println!("{:?}", res);
}
