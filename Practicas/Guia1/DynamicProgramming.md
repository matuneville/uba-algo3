# Guía 1: ejercicios de Dynamic Programming

## Ejercicio 5



## Ejercicio 6

### Ítem A 

- (inf, inf) si i = |B| y j > 0  

- (0, 0) si j <= 0  

- min( (cc({b2,...,bn}, c-b1).first + b1, 1 + cc({b2,...,bn}, c-b1).second),
     (cc({b2,...,bn}, c).first, cc({b2,...bn}, c).second)) caso contrario

### Ítem C

```rust
pub fn tragabilletes(billetes: & Vec<i8>, i: usize, j: i8) -> (i8, i8){
    // i = indice que veo de billetes, j = precio restante
    // return (cuanto_pago, #billetes_usados)
    if j <= 0 {
        return (0, 0);
    }
    if i == billetes.len(){ // solo llega aca si no llego al precio requerido
        return (127, 127); // (infinito - infinito)
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
```

La complejidad es $O(2^n)$ porque es el producto de la cantidad total de nodos ($O(2^{n+1}-1)$) y la complejidad de calcular cada uno (constante).  

- La cantidad de *estados posibles* es $n \times 2^n$ (porque $i$ puede tomar $n$ valores posibles, siendo $n$ la cantidad de billetes, mientras que $j$ puede tomar $2^n$ valores posibles)
- La cantidad total de *llamados recursivos* es $O(2^n)$

No hay superposición de estados ¿?¿?

## Ejercicio 7

### Ítem B

av(P, c, j) es  

- indefinido si c < 0 o c > j >= |P|

- 0 si j < 0 (indexo desde 0)

- max( av(P, j-1, c+1) + P[j], av(P, j-1, c-1) - P[j], av(P, j-1, c) )


