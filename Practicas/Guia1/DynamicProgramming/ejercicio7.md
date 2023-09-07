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
