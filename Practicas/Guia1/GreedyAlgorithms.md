# Guía 1: Ejercicios de algoritmos greedy

## Ejercicio 13

### Ítem B

```rust
pub fn parejas(grupo_a: Vec<i32>, grupo_b: Vec<i32>) -> i32 {
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
```

## Ejercicio 14

### Ítem A

Un algoritmo sencillo podría ordenar el conjunto (representado mediante un arreglo arreglo) de forma decreciente, y luego tomar los $k$ primer elementos de este, que serán los más grandes del arreglo.  

#### Demostración

Con el algoritmo propuesto, podemos decir que:

$$
maxSuma = \sum_{i=0}^{k-1}sorted(X)[i] = \sum_{\forall e\in S} S
$$

Es decir, los $k$ primer elementos del arreglo serán los mayores, que maximizarán la suma posible tomando subconjuntos de cardinal $k$.  

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

- LLevamos el arreglo a heap con un algoritmo **_up-heapify_** o **_sift-up_**, que cuesta $O(n)$
- Luego, vamos extrayendo los $k$ elementos
- La extracción de cada uno cuesta $O(log\ n)$ por el _sift-down_ para reacomodar el resto del heap.
- Complejidad final: $O(k\ log\ n)$, pues estaremos realizando $k$ veces el proceso mencionado previamente (_ejercicio muy algo2 jeje_)







