// ############################## ejercicio 13 ############################## ##############################

pub fn parejas(grupo_a: & Vec<i32>, grupo_b: & Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    let mut i_a: usize = 0;
    let mut i_b: usize = 0;

    while i_a < grupo_a.len() && i_b < grupo_b.len(){
        if grupo_a[i_a] - grupo_b[i_b] <= 1 && grupo_a[i_a] - grupo_b[i_b] >= - 1{
            result += 1;
            i_a += 1;
            i_b += 1;
        }
        else if grupo_a[i_a] < grupo_b[i_b]{
            i_a += 1;
        }
        else{
            i_b += 1;
        }
    }
    return result;
}

// ############################## ejercicio 14 ############################## ##############################

pub fn max_k_sum(mut s: Vec<i32>, k: usize) -> i32{
    let mut res = 0;
    s.sort_by(|a, b| b.cmp(a)); // ordeno decrecientemente

    for i in 0..k{
        res += s[i];
    }
    return res;
}

// ############################## ejercicio 16 ############################## ##############################

pub fn gasolineras(p: & Vec<i32>, mut c: i32) -> Vec<i32>{
    let mut sol: Vec<i32> = vec![];
    let c_og = c;
    for i in 0..p.len()-1{
        if p[i+1]-p[i] <= c {
            c = c - (p[i+1]-p[i]); // todavia alcanza asi que gasto nafta hasta sgte parada
        }
        else{
            sol.push(p[i]);
            c = c_og - (p[i+1]-p[i]); // cargo nafta y gasto hasta la sgte parada
        }
    }
    return sol;
}