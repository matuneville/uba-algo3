## Ejercicio 14

### Ítem A

Un algoritmo sencillo podría ordenar el conjunto (representado mediante un arreglo arreglo) de forma decreciente, y luego tomar los $k$ primer elementos de este, que serán los más grandes del arreglo.  

#### Demostración (media informal je)

Con el algoritmo propuesto, podemos decir que:

$$
maxSuma = \sum_{i=0}^{k-1}sorted(X)[i] = \sum_{\forall e\in S} S
$$

Es decir, los $k$ primer elementos del arreglo serán los mayores, que maximizarán la suma posible tomando subconjuntos de cardinal $k$. Dicho de otra forma, estaremos tomando el máximo elemento del conjunto $k$ veces.  

Este algoritmo es correcto ya que genera una solución válida (la suma de un subconjunto de $k$ elementos, pues estamos extrayendo al máximo dicha cantidad de veces) y es la máxima suma posible.   

Supongamos entonces que existe una posible mayor suma que la que obtuvimos con dicho algoritmo. Esto significaría que existe un elemento que se encuentra entre los últimos $n-k$ elementos del arreglo, que, si lo reemplazaramos por uno de los $k$ primeros, nos daría una mayor suma. Esto solo sería posible si este elemento fuera mayor que el que fue reemplazado, pero esto es imposible pues incluso el último elemento de los $k$ mayores es mayor que todo el resto. Es decir, $\forall i\in \mathbb{N},\ k \leq i < n \Rightarrow S[k-1] > S[i]$.

### Ítem B

```rust
pub fn max_k_sum(mut s: Vec<i32>, k: usize) -> i32{
    let mut res = 0;
    s.sort_by(|a, b| b.cmp(a)); // ordeno decrecientemente

    for i in 0..k{ // sumo los primeros k
        res += s[i];
    }
    return res;
}
```

La complejidad de este algoritmo con su sorting y suma de $k$ elementos es de $O(n\ log\ n + k)$ y como $k$ está acotado por $n$, $\Rightarrow O(n\ log\ n)$

### Ítem C

Hay una forma más eficiente con la que no necesitaremos ordenar todo el arreglo:

- Agarro los primeros $k$ elementos del arreglo y armo un min heap usando **_heapify_**, que costará $O(k)$
- Luego comienzo a ver los $n-k$ elemenos que me sobraron, y voy comparando con la cabeza del heap
- Si es mayor que el elemento minimo, entonces lo quito y lo agrego, que cuesta $O(log\ k)$
- En el peor caso lo haré con los $n-k$ elementos
- La complejidad queda entonces $O(k + (n-k)\ log\ k) = O((n-k)\ log\ k) \in O(n\ log\ k)$ y listo (_ejercicio muy algo2 jeje_)