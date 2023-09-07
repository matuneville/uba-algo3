## Ejercicio 3


Una forma de resolverlo sería buscar todos los subconjuntos de $k$ elementos en el conjunto de naturales $(1...n)$, cuya cantidad coincidirá con el número combinatorio $nCk$. Habría que buscarlos de manera exhaustiva y tener una variable constantemente que nos diga cuál es la máxima suma total posible. El árbol de desiciones de búsqueda de subconjuntos quedaría así: ejemplo con $k=$ y $n=5$  

![a3](/Practicas/Guia1/arboles/ej3.png)

Luego de obtener cada subconjunto, solo quedaría ver qué suma maximiza y chequear si es una nueva solución. Hice el algoritmo directamente con la mejor poda que se me ocurrió, que es lo que se ve en el árbol. La solución en código es la siguiente:

```rust
pub fn index_subset_max(matrix: & Vec<Vec<usize>>, k: usize){

    let mut n_set: Vec<usize> = vec![];
    for i in 0..matrix.len(){ n_set.push(i); } // aca inicializo todo lo necesario para la recursión
    let indexes: Vec<usize> = vec![];
    let mut solution: Vec<usize> = vec![];
    let mut sum: usize = 0;

    fn index_subset_max_helper(matrix: & Vec<Vec<usize>>,
                               indexes: & Vec<usize>,
                               current_solution: &mut Vec<usize>,
                               max_sum: &mut usize,
                               k: usize,
                               index: usize,
                               n_set: &Vec<usize>)
    {
        if k == 0 {
            println!("Formando subconjunto {:?}", indexes); // print test subset
            let new_sum = ejercicios_help::sum_up_all(matrix, indexes);
            if new_sum > *max_sum {
                *current_solution = indexes.clone();
                *max_sum = new_sum;
            }
        }
        else {
            // este ciclo restringe mucho la recursion para evitar conjuntos que no llevan a nada gracias al 1...(n - k)
            for i in index..=n_set.len() - k  { // si sacamos el -k funcionará igual, pero se harán más operaciones
                let mut new_solution = indexes.clone();
                new_solution.push(i);
                println!("Formando subconjunto {:?}", indexes); // test
                index_subset_max_helper(& matrix,
                                        &mut new_solution,
                                        current_solution,
                                        max_sum,
                                        k - 1,
                                        i+1,
                                        & n_set);
            }
        }
    }

    index_subset_max_helper(matrix, &indexes, &mut solution, &mut sum, k, 0, & n_set);

    println!("La solución es {:?} que maximiza la suma {}", solution, sum)
}
```

Acá el test de la guía:

```rust
let m: Vec<Vec<usize>> = vec![
                                vec![0,10,10,1],
                                vec![0,0,5,2],
                                vec![0,0,0,1],
                                vec![0,0,0,0]];

    index_subset_max(&m, 3);
```

Output:
```
Formando subconjunto []
Formando subconjunto [0]
Formando subconjunto [0, 1]
Formando subconjunto [0, 1, 2]
Formando subconjunto [0, 1]
Formando subconjunto [0, 1, 3]
Formando subconjunto [0]
Formando subconjunto [0, 2]
Formando subconjunto [0, 2, 3]
Formando subconjunto []
Formando subconjunto [1]
Formando subconjunto [1, 2]
Formando subconjunto [1, 2, 3]
La solución es [0, 1, 2] que maximiza la suma 25
```

La guía indexa a partir de 1 en vez de 0, por eso varían los números, pero es lo mismo. Le agregué un print para ver cómo se va formando el subconjunto y apreciar cómo el algoritmo no crea subconjuntos que no terminan en ninguna solución, como $[2,3]$, al que no se le podría agregar ningun otro elemento.  