mod ejercicios_dp;
use crate::ejercicios_dp::tragabilletes;

fn main() {
    let billetes = vec![20, 10, 5, 3, 2,20];
    let res = tragabilletes(& billetes, 0, 14);
    println!("{:?}", res);
}
