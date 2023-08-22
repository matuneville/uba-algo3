mod ejercicios;
use ejercicios::*;

fn main() {

    let m: Vec<Vec<usize>> = vec![
                                    vec![0,10,10,1],
                                    vec![0,0,5,2],
                                    vec![0,0,0,1],
                                    vec![0,0,0,0]];

    index_subset_max(&m, 3);
}
