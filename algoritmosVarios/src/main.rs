mod ejercicios;
use ejercicios::*;

fn main() {

    /*let m: Vec<Vec<usize>> = vec![
        vec![3,2,2,1],
        vec![0,14,5,14],
        vec![9,0,0,0],
        vec![9,13,10,0]];*/

    let n: Vec<Vec<usize>> = vec![
        vec![3,2,2,1,6,23],
        vec![0,14,5,14,0,1],
        vec![29,0,0,0,0,3],
        vec![9,13,10,0,1,5],
        vec![0,3,2,4,1,5],
        vec![2,3,10,5,10,8]];

    /*let m: Vec<Vec<usize>> = vec![
                                    vec![0,10,10,1],
                                    vec![0,0,5,2],
                                    vec![0,0,0,1],
                                    vec![0,0,0,0]];*/

    index_subset_max(&n, 4);
}
