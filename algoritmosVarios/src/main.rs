mod ejercicios_dp;
use crate::ejercicios_dp::{tragabilletes_topdown, tragabilletes_bt};

fn main() {
    let billetes = vec![4,3,2,9,6,7,8,2,3];

    let precio = 17;

    let res = tragabilletes_bt(& billetes, 0, precio);
    let res2 = tragabilletes_topdown(&billetes, 0, precio);

    println!("{:?}", res);
    println!("{:?}", res2);
}
