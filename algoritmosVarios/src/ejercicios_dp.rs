use std::cmp::min;

pub fn tragabilletes(billetes: & Vec<i8>, i: usize, j: i8) -> (i8, i8){
    // i = indice que veo de billetes, j = precio restante
    // return (cuanto_pago, #billetes_usados)
    if j <= 0 {
        return (0, 0);
    }
    if i == billetes.len(){ // solo llega aca si no llego al precio requerido
        return (127, 127);
    }

    let bt_con_billete = tragabilletes(billetes, i+1, j-billetes[i]); // ($5, 1)
    let bt_sin_billete = tragabilletes(billetes, i+1, j); // ($5, 2)

    let res = if bt_sin_billete.0 == bt_con_billete.0 {
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