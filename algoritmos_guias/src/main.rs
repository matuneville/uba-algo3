mod guia1 {pub mod ejercicios_greedy; }
use guia1::ejercicios_greedy::*;

fn main() {
    let s = vec![0,3,4,7,10,11,16,21,25,26,27,30];

    println!("{:?}", gasolineras(& s, 5));
}
