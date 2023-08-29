// #########################  EJERCICIO 6 ######################### ######################### #########################

use std::arch::x86_64::_bittest;
use std::cmp::max;

pub fn tragabilletes_bt(billetes: & Vec<i32>, i: usize, j: i32) -> (i32, i32){
    // i = indice que veo de billetes, j = precio restante
    // return (cuanto_pago, #billetes_usados)
    if j <= 0 {
        return (0, 0);
    }
    if i == billetes.len(){ // solo llega aca si no llego al precio requerido
        return (999, 999); // representa infinito infinito, no deberia usar numeros tan grandes
    }

    let bt_con_billete = tragabilletes_bt(billetes, i+1, j-billetes[i]);
    let bt_sin_billete = tragabilletes_bt(billetes, i+1, j);

    let res = if bt_sin_billete.0 == bt_con_billete.0 + billetes[i] {
                        if bt_con_billete.1 < bt_sin_billete.1 { (bt_con_billete.0 + billetes[i], bt_con_billete.1 + 1) }
                        else { bt_sin_billete }
                    }
                    else if bt_sin_billete.0 < bt_con_billete.0 + billetes[i] {
                        bt_sin_billete
                    }
                    else {
                        (bt_con_billete.0 + billetes[i], bt_con_billete.1 + 1)
                    };
    return res;
}


pub fn tragabilletes_topdown(billetes: & Vec<i32>, i: usize, j: i32) -> (i32, i32) {

    let mut matriz_dp = crear_matriz_memo(billetes.len(), j); // armo el cuadrado para memorizar

    fn billetes_topdown_recursivo(billetes: &Vec<i32>, memo_bottom_up: &mut Vec<Vec<(i32, i32)>>, i: usize, j: i32) -> (i32, i32) {

        if j <= 0 { return (0, 0); }

        if i == billetes.len() { return (999, 999); }

        if memo_bottom_up[i][j as usize] != (-1, -1) { // si ya esta resuelto
            return memo_bottom_up[i][j as usize];
        }

        let bt_con_billete = billetes_topdown_recursivo(billetes, memo_bottom_up, i + 1, j - billetes[i]);
        let bt_sin_billete = billetes_topdown_recursivo(billetes, memo_bottom_up, i + 1, j);

        memo_bottom_up[i][j as usize] = if bt_sin_billete.0 == bt_con_billete.0 + billetes[i] { // modifico matriz memo
            if bt_con_billete.1 < bt_sin_billete.1 { (bt_con_billete.0 + billetes[i], bt_con_billete.1 + 1) } else { bt_sin_billete }
        } else if bt_sin_billete.0 < bt_con_billete.0 + billetes[i] {
            bt_sin_billete
        } else {
            (bt_con_billete.0 + billetes[i], bt_con_billete.1 + 1)
        };

        return memo_bottom_up[i][j as usize];
    }

    return billetes_topdown_recursivo(& billetes, &mut matriz_dp, i, j);
}

pub fn crear_matriz_memo(n: usize, p: i32) -> Vec<Vec<(i32,i32)>>{
    let mut memo_topdown: Vec<Vec<(i32, i32)>> = vec![];
    for _ in 0..n {
        let mut linea: Vec<(i32, i32)> = vec![];
        for _ in 0..=p{
            linea.push((-1,-1));
        }
        memo_topdown.push(linea);
    }
    return memo_topdown;
}

/*pub fn tragabilletes_bottomup(billetes: & Vec<i32>, j: i32) -> (i32, i32) {
    let n: usize = billetes.len();

    // inicio la matriz
    let matriz_dp: Vec<Vec<(i32, i32)>> = vec![];

    // inicio casos base
    for i in 0..=n {
        matriz_dp[i][0] = (0, 0);
    }



    for i in 0..n{
        for c in (0..=j).rev() {
            let r = matriz_dp[i][c - billetes[i]];
        }
    }

    return (0,0);
}*/


// #########################  EJERCICIO 7 ######################### ######################### #########################

pub fn astro_void(asteroides: & Vec<i32>) -> i32 {
    fn astro_recursivo(asteroides: &Vec<i32>, cant: i32, dia: i32) -> i32 {


        if cant-1 > dia || cant < 0 { return -99999; } // - infinito

        if dia < 0 { return 0; } // caso base

        let res = max(astro_recursivo(asteroides, cant - 1, dia - 1) - asteroides[dia as usize], // caso vendo
                      max(astro_recursivo(asteroides, cant + 1, dia - 1) + asteroides[dia as usize], // caso compro
                          astro_recursivo(asteroides, cant, dia - 1))); // caso vendo y compro, o no hago nada

        return res;
    }
    return astro_recursivo(&asteroides, 0, (asteroides.len() - 1) as i32);
}

pub fn astro_dp_topdown(asteroides: & Vec<i32>) -> i32 {

    let mut dp = crear_matriz_astro(asteroides.len(), asteroides.len()); // creo matriz para dp

    fn astro_recursivo(asteroides: &Vec<i32>, dp: &mut Vec<Vec<i32>>, cant: i32, dia: i32) -> i32 {

        let cant_i: usize = cant as usize; // uso otro type para indexar
        let dia_i: usize = dia as usize;

        if cant-1 > dia || cant < 0 { return -99999; } // caso base -infinito

        if dia < 0 { return 0; } // caso base 0

        if dp[dia_i][cant_i] != -1 { return dp[dia_i][cant_i]; }

        dp[dia_i][cant_i] = max(astro_recursivo(asteroides, dp, cant - 1, dia - 1) - asteroides[dia_i], // caso vendo
                      max(astro_recursivo(asteroides, dp, cant + 1, dia - 1) + asteroides[dia_i], // caso compro
                          astro_recursivo(asteroides, dp, cant, dia - 1))); // caso vendo y compro, o no hago nada

        return dp[dia_i][cant_i];
    }
    return astro_recursivo(&asteroides, &mut dp, 0, (asteroides.len() - 1) as i32);
}

pub fn crear_matriz_astro(n: usize, p: usize) -> Vec<Vec<i32>>{
    let mut memo_topdown: Vec<Vec<i32>> = vec![];
    for _ in 0..n {
        let mut linea: Vec<i32> = vec![];
        for _ in 0..=p{
            linea.push(-1);
        }
        memo_topdown.push(linea);
    }
    return memo_topdown;
}

pub fn astro_dp_bottomup(asteroides: & Vec<i32>) -> i32 {
    let mut dp = crear_matriz_astro(asteroides.len()+1, asteroides.len()+1); // creo matriz para dp
    let n = asteroides.len();

    for i in 0..=n{
        dp[0][i] = 0; // el dia base con 0 ganancia, seria como el dia -1
    }

    for dia in 1..n{
        for cant in 0..dia+1{
            let r1 = dp[dia - 1][cant - 1] + asteroides[dia - 1];
            let r2 = dp[dia - 1][cant + 1] - asteroides[dia - 1];
            let r3 = dp[dia - 1][cant];
            dp[dia][cant] = max(r1, // caso vendo
                                max(r2, // caso compro
                                    r3)) // caso vendo y compro, o no hago nada
        }
    }

    let mut max_ganancia = 0;
    for cant in 0..n {
        max_ganancia = max(max_ganancia, dp[n - 1][cant]);
    }

    max_ganancia
}

// #########################  EJERCICIO 10 ######################### ######################### #########################

pub fn apilar_cajas(w: & Vec<i32>, s: & Vec<i32>) -> i32 {

    fn apilar_recursivo(w: & Vec<i32>, s: & Vec<i32>, soporte: i32, i: usize, sol_actual: i32, sol_max: i32){
        if soporte < 0{
            
        }

    }
}







