## Ejercicio 10

### Ítem B

$$
f(soporte, i)_{W,S} =
\begin{cases}
     0 & \text{si } i = |P| \\
     -\infty & \text{si } soporte < 0 \\
     max(f(min(soporte-W[i], S[i]), i+1) + 1, f(soporte, i+1) ) & \text{caso contrario}
\end{cases}
$$

Para hacer el algoritmo en top-down, lo llamo con $soporte = suma(S)$, es decir, con la suma de todos los soportes, para tener una forma con la que llamarlo

```rust
pub fn apilar_topdown(w: & Vec<i32>, s: & Vec<i32>) -> i32{
    let mut suma_soportes = 0;
    for sop in s{
        suma_soportes += *sop;
    }
    let mut dp = crear_matriz_cajas(w.len()+1, suma_soportes+1);

    fn apilar_r(w: & Vec<i32>, s: & Vec<i32>, dp: &mut Vec<Vec<i32>>, soporte: i32, i: usize, suma_soportes: i32) -> i32 {
        if i == w.len(){ return 0; } // caso base: considere todas las cajas
        else if soporte < 0 { return -99999; } // termine de poner cajas

        let soporte_con = if soporte == suma_soportes { s[i] } else { min(soporte - w[i], s[i]) };
        let soporte_sin = if soporte == suma_soportes { suma_soportes }   else { soporte };

        let sop_i = soporte as usize; // cambio tipo para indexar

        if dp[i][sop_i] == -1 {

            let res = max(
                            apilar_r(&w, &s, dp, soporte_con, i + 1, suma_soportes) + 1,
                            apilar_r(&w, &s, dp, soporte_sin, i + 1, suma_soportes));
            dp[i][sop_i] = res;
        }

        return dp[i][sop_i];
    }

    return apilar_r(& w, & s, &mut dp, suma_soportes, 0, suma_soportes);
}
```