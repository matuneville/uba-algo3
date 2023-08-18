# Guía práctica 1: BackTracking

## Ejercicio 1

### Item A

Las soluciones _candidatas_ son $(0,0,0), (0,0,1), (0,1,0), (0,1,1), (1,0,0), (1,0,1), (1,1,0)$ y $(1,1,1)$

### Item B

Las soluciones _válidas_ son $(0,1,0)$ y  $(1,0,1)$

### Item C

Las soluciones _parciales_ son los subconjuntos del conjunto de partes de $(6,12,6)$

### Item D

El árbol de desiciones con _backtracking_ quedaría así:

![im1](/Practicas/Guia1/arboles/ej1.png)

Los nodos verdes serían las soluciones válidas. Los nodos rojos serían los inválidos, de los que no habría que continuar desarrollando las desiciones

### Item F

### Item G

Árbol de desiciones del algoritmo propuesto en ítem $f)$

![im2](arboles/ej1g.png)


### Item H

Árbol de desiciones del algoritmo propuesto con la _regla de factibilidad_

![im2](arboles/ej1h.png)

### Item I

Podemos añadir la condición
```
    Si j == 0, retornar True
```

Cuando $j=0$, entonces en esa ramificación del árbol habrá una solución válida (o el mismo nodo será la solución). El árbol entonces quedaría:

![im3](arboles/ej1i.png)

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

## Ejercicio 2

### Item A

Habría que generar $(n^2)!$ cuadrados mágicos si usáramos un algoritmo de _brute force_.


## Ejercicio 3

### Item A


Pseudocódigo:
```

```
