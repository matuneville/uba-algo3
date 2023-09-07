## Ejercicio 2

### Ítem A

Habría que generar $(n^2)!$ cuadrados mágicos si usáramos un algoritmo de _brute force_.

### Ítem B

El árbol recursivo quedaría de la forma:

![im5](/Practicas/Guia1/arboles/ej2b.png)

Un algoritmo sería ir poniendo los números que corresponden; podemos tener un conjunto de $1...n$ en el que vamos quitando los elementos que ya pusimos, así nos ahorramos tener elementos repetidos y chequear en cada nodo no hoja que se cumpla esto. Luego, cuando este conjunto esté vacío, quiere decir que ya pusimos todos los números posibles, que ya llenamos el cuadrado y estamos en un nodo hoja; en ese caso, corroboraremos si el cuadrado cumple su propiedad de ser mágico. La complejidad temporal será de $O((n^2)! \times n^2)$  

El algoritmo en `Rust` para hallar los cuadrados mágicos de orden $n$ queda:

```rust
// busco cuadrados magicos con brute force
pub fn magic_squares(n: u8){

    fn find_all_squares(square: Vec<Vec<u8>>, nums: Vec<u8>, pos: (usize, usize)) -> Vec<Vec<Vec<u8>>> {

        if nums.is_empty() {
            let mut result: Vec<Vec<Vec<u8>>> = Vec::new();
            result.push(square.clone());
            result

        } else {
            let mut results: Vec<Vec<Vec<u8>>> = Vec::new();
            for i in 0..nums.len() {
                let mut new_square = square.clone();
                new_square[pos.0][pos.1] = nums[i]; // square with new num

                let mut new_nums = nums.clone();
                new_nums.remove(i); // erase inserted num

                let new_pos = // new position to do recursion
                    if pos.1 == square.len() - 1 {
                    (pos.0 + 1, 0) } else { (pos.0, pos.1 + 1)
                };
                results.extend(find_all_squares( // recursion
                    new_square,
                    new_nums,
                    new_pos,
                ));
            }
            results
        }
    }       // lo que sigue es solo para printear en pantalla las soluciones

    let initial_square = vec![vec![0; n as usize]; n as usize];
    let initial_nums = (1..=n*n).collect::<Vec<u8>>();
    let result = find_all_squares(initial_square, initial_nums, (0, 0));

    for sq in result{
        if is_magic(&sq) { println!("{:?}", sq) }
    }
}
```

Un test:

```rust
fn main() {
    ejercicios::magic_squares(3);
}
```

Output:
```
[[2, 7, 6], [9, 5, 1], [4, 3, 8]]
[[2, 9, 4], [7, 5, 3], [6, 1, 8]]
[[4, 3, 8], [9, 5, 1], [2, 7, 6]]
[[4, 9, 2], [3, 5, 7], [8, 1, 6]]
[[6, 1, 8], [7, 5, 3], [2, 9, 4]]
[[6, 7, 2], [1, 5, 9], [8, 3, 4]]
[[8, 1, 6], [3, 5, 7], [4, 9, 2]]
[[8, 3, 4], [1, 5, 9], [6, 7, 2]]
```

## Ítem E

El árbol recursivo quedaría entonces de la siguiente forma:  

![im2d](/Practicas/Guia1/arboles/ej2d.png)  

Lo que hará entonces mi nueva solución es, cada vez que haya una línea completa en el cuadrado (sea row, column o diagonal) entonces chequeará que su suma sea igual al número mágico.  

Para le nuevo algoritmo, paso un vector con las soluciones por referencia mutable, para que decida el algoritmo si añadir o no la solución (a diferencia de antes, que al ser fuerza bruta toda rama de desiciones era una solución). En este caso la función no tiene ningun tipo de retorno. El código quedaría:

```rust
pub fn magic_squares_bt(n: u8){

    let magic_num = (n*n*n + n)/2;

    fn find_magic_squares(result: &mut Vec<Vec<Vec<u8>>>, square: Vec<Vec<u8>>, nums: Vec<u8>, pos: (usize, usize), magic_num: &u8) {

        // #############################################################################################
        if ! ejercicios_help::check_full_lines(&square, &magic_num){ // NUEVA REGLA DE FACTIBILIDAD !!!!
            return;                                                  
        } // ###########################################################################################

        if nums.is_empty() {
            result.push(square.clone());
            return;

        } else {
            for i in 0..nums.len() {
                let mut new_square = square.clone();
                new_square[pos.0][pos.1] = nums[i]; // square with new num

                let mut new_nums = nums.clone();
                new_nums.remove(i); // erase inserted num

                let new_pos = // new position to do recursion
                    if pos.1 == square.len() - 1 {
                        (pos.0 + 1, 0) } else { (pos.0, pos.1 + 1)
                    };
                find_magic_squares( // recursion
                                    result,
                                    new_square,
                                    new_nums,
                                    new_pos,
                                    magic_num);
            }
        }
    }       // lo que sigue es solo para printear en pantalla las soluciones

    let mut result: Vec<Vec<Vec<u8>>> = vec![];
    let initial_square = vec![vec![0; n as usize]; n as usize];
    let initial_nums = (1..=n*n).collect::<Vec<u8>>();
    find_magic_squares(&mut result, initial_square, initial_nums, (0, 0), &magic_num);

    for sq in result {
        println!("{:?}", sq)
    }
}
```

*Aclaraciones: intenté hacer algo distinto, como un algoritmo que cuando ya se pasara con los primeros numeros (por ejemplo, que la primera linea fuera [9,8,0] y todo el resto tuviera 0s) ya cortara la rama recursiva, pero estuve horas intentando hacer algo que al final estaba mal, así que me terminó dando fiaca e implemente algo más simple. Por último, hay una forma de hacerlo sin utilizar el conjunto de lso números restantes por ubicar, pero no sería con el invariante que propone el ejercicio por lo que fue la solución más factible que se me ocurrió.*  