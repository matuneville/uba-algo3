# Guía 1: ejercicios de Dynamic Programming

## Ejercicio 5



## Ejercicio 6

### Ítem A 

$$
cc(B, c) =
\begin{cases}
     (\infty, \infty) \ \ \ \ \  \ \ \ \ \ \ \ \ \ \   \text{si } i=|B| \land j > 0\\
     (0, 0)     \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \ \  \text{si } j \leq 0\\
     min( (cc([b_2,...,b_n], c-b_1).first + b_1, 1 + cc([b_2,...,b_n], c-b_1).second), (cc([b_2,...,b_n], c).first, cc([b_2,...,b_n], c).second))\ \text{caso contrario}
\end{cases}
$$

El $min(tupla1, tupla2)$ lo que hace es tomar la tupla con el menor primer elemento. Si coinciden, se fija en el segundo.

### Ítem C

Hice el algoritmo directamente como lo pide en el ítem C. La diferencia con el B es que en este el arreglo de billetes no se inmuta, por lo que su complejidad temporal es mejor.  

`billetes` funciona como variable global pues no se modifica:

```rust
pub fn tragabilletes_bt(billetes: & Vec<i32>, i: usize, j: i32) -> (i32, i32){
    // i = indice que veo de billetes, j = precio restante
    // return (cuanto_pago, #billetes_usados)
    if j <= 0 {
        return (0, 0);
    }
    if i == billetes.len(){ // solo llega aca si no llego al precio requerido
        return (99999, 99999); // representa infinito infinito, no deberia usar numeros tan grandes
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
```

La complejidad es $O(2^n)$ porque es el producto de la cantidad total de nodos ($O(2^{n+1}-1)$ ) y la complejidad de calcular cada uno (constante).   

#### Superposición de estados:

- La cantidad de *estados posibles* es $n \times P$ (porque $i$ puede tomar $n$ valores posibles, siendo $n$ la cantidad de billetes, mientras que $j$ puede tomar $2^n$ valores posibles)
- La cantidad total de *llamados recursivos* es $O(2^n)$

- $nP <<< 2^n$. Hay superposición de estados pues la cantidad de estados posibles es considerablemente menor que la cantidad de llamados recursivos (para $P$ no tan grande).

### Ítem D

Hay que usar una matriz de tamaño $n \times P$ para memoizar el algoritmo de backtracking y hacer programación dinámica.

### Ítem E

`billetes` y `memo_bottom_up` funcionan como variables globales

```rust
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
```

### Ítem F

La **complejidad temporal** se define como $\text{cantidad de estados posibles} \times \text{calcular cada nodo}$. Por lo tanto, la complejidad temporal es de $O(n\times P \times 1) = O(nP)$

### Ítem G

```
ni idea despues lo hago
```


## Ejercicio 7

### Ítem B

$$
av(P, c, j) =
\begin{cases}
     -\infty & \text{si } c < 0 \lor |P| \leq j < c \\
     0 & \text{si } j < 0 \\
     max(av(P, j-1, c+1) + P[j], av(P, j-1, c-1) - P[j], av(P, j-1, c)) & \text{caso contrario}
\end{cases}
$$

### Ítem D

```rust
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
```

Su **complejidad temporal** está dada por $\text{cantidad de estados posibles} \times \text{calcular cada nodo}$, que da como resultado $O(dias \times asteroides)$, que con las variables dadas por la consigna quedaría en $O(j\times c)$.  

La **complejidad espacial** está dada por la matriz de memoización de tamaño $j^2$, y las $k$ veces en las que se guardan copias de $j$ y $c$, por lo que la complejidad queda $O(j^2 + k)$


### Ítem E

```
```


## Ejercicio 10

### Ítem B

$$
f(soporte, i)_{W,S} =
\begin{cases}
     0 & \text{si } i = |P| \\
     -\infty & \text{si } soporte < 0 \\
     max(f(min(soporte-S[i], S[i]), i+1) + 1, f(soporte, i+1) ) & \text{caso contrario}
\end{cases}
$$