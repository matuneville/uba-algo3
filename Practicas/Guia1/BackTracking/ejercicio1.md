## Ejercicio 1

### Ítem A

Las soluciones _candidatas_ son $(0,0,0), (0,0,1), (0,1,0), (0,1,1), (1,0,0), (1,0,1), (1,1,0)$ y $(1,1,1)$

### Ítem B

Las soluciones _válidas_ son $(0,1,0)$ y  $(1,0,1)$

### Ítem C

Las soluciones _parciales_ son los subconjuntos del conjunto de partes de $(6,12,6)$

### Ítem D

El árbol de desiciones con _backtracking_ quedaría así:

![im1](/Practicas/Guia1/arboles/ej1.png)

Los nodos verdes serían las soluciones válidas. Los nodos rojos serían los inválidos, de los que no habría que continuar desarrollando las desiciones

### Ítem F

La **complejidad temporal** viene dada por: $cantidad\ hojas \times O(c/nodo) \Rightarrow O(2^n)$  

La **complejidad espacial** viene dada por: $O(mantener\ rama\ recursiva) \Rightarrow O(n)$  

### Ítem G

Árbol de desiciones del algoritmo propuesto en ítem $f)$

![im2](/Practicas/Guia1/arboles/ej1g.png)


### Ítem H

Árbol de desiciones del algoritmo propuesto con la _regla de factibilidad_

![im2](/Practicas/Guia1/arboles/ej1h.png)

### Ítem I

Podemos añadir la condición
```
    Si j == 0, retornar True
```

Cuando $j=0$, entonces en esa ramificación del árbol habrá una solución válida (o el mismo nodo será la solución). El árbol entonces quedaría:

![im3](/Practicas/Guia1/arboles/ej1i.png)

El algoritmo implementado en `Rust` que cuenta la cantidad de soluciones queda:

```rust
pub fn subset_sum(nums: Vec<i32>, k: i32) -> i32 {
    fn subset_sum_helper(nums: &Vec<i32>, i: usize, k: i32) -> i32 {

        if k == 0 { 1 } // con este caso recorto mas el arbol

        else if i == 0 {
            if k == nums[i] { 1 }
            else { 0 }
        }
        else{
            subset_sum_helper(nums, i-1, k) +
                subset_sum_helper(nums, i-1, k-nums[i])
        }
    }
    subset_sum_helper(&nums, nums.len()-1, k)
}
```
Esto **sólo es válido porque el multiconjuntos es de Naturales**, ya que, quizá cortamos el algoritmo cuando nuestro $k$ llegó a $0$, por lo que nó habría más naturales para restarle. En cambio si estuviera como elemento el $0$, también formaría parte del subconjunto, y habrían más posibilidades.

### Ítem J

Me da fiaca hacer este, pero bastaría con una modificación en el algoritmo previo, teniendo un vector mutable en el argumento al que voy agregando las soluciones.