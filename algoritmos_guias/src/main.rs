mod guia1 {pub mod ejercicios_dp; }
use guia1::ejercicios_dp::*;

fn main() {
    let s = vec![3, 2, 5, 6];

    println!("{:?}", astro_dp_topdown(& s));
}
