mod ejercicios_dp;
use crate::ejercicios_dp::{tragabilletes_topdown, tragabilletes_bt, astro_void, astro_void_dp};

fn main() {
    let asteroides = vec![3, 2, 5, 6];

    let res = astro_void_dp(& asteroides);

    println!("{:?}", res);
}
