// #########################  EJERCICIO 6 ######################### ######################### #########################

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

    let mut matriz_memo = crear_matriz_memo(billetes.len(), j); // armo el cuadrado para memorizar

    fn bottomup_recursivo(billetes: &Vec<i32>, memo_bottom_up: &mut Vec<Vec<(i32, i32)>>, i: usize, j: i32) -> (i32, i32) {
        if j <= 0 { return (0, 0); }
        if i == billetes.len() { return (127, 127); }

        if memo_bottom_up[i][j as usize] != (-1, -1) { // si ya esta resuelto
            return memo_bottom_up[i][j as usize];
        }

        let bt_con_billete = bottomup_recursivo(billetes, memo_bottom_up, i + 1, j - billetes[i]);
        let bt_sin_billete = bottomup_recursivo(billetes, memo_bottom_up, i + 1, j);

        memo_bottom_up[i][j as usize] = if bt_sin_billete.0 == bt_con_billete.0 + billetes[i] { // modifico matriz memo
            if bt_con_billete.1 < bt_sin_billete.1 { (bt_con_billete.0 + billetes[i], bt_con_billete.1 + 1) } else { bt_sin_billete }
        } else if bt_sin_billete.0 < bt_con_billete.0 + billetes[i] {
            bt_sin_billete
        } else {
            (bt_con_billete.0 + billetes[i], bt_con_billete.1 + 1)
        };

        return memo_bottom_up[i][j as usize];
    }

    return bottomup_recursivo(& billetes, &mut matriz_memo, i, j);
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

// #########################  EJERCICIO 7 ######################### ######################### #########################