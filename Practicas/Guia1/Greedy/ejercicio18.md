## Ejercicio 18

A ojo se ve fácilmente que una solución es ir tomando los distintos subconjuntos de manera ordenada para obtener la mayor sumatoria de $mex(C)$. Llamo $f_{mex}$ a la función propuesta por el ejercicio. Por ejemplo, en el vector $\lbrace3, 0, 1\rbrace$, si lo ordenamos como $\lbrace 1, 0, 3\rbrace$, vamos a obtener:

$$
f_{mex}(\lbrace 1, 0, 3\rbrace) =
$$
 
$$
mex(\lbrace 1 \rbrace) + mex(\lbrace 1, 0\rbrace) + mex(\lbrace 1, 0, 3\rbrace) = 0 + 2+ 2 = 4
$$

Mientras que si lo ordenamos crecientemente:

$$
f_{mex}(\lbrace 0, 1, 3\rbrace) =
$$
 
$$
mex(\lbrace 0 \rbrace) + mex(\lbrace 0, 1\rbrace) + mex(\lbrace 0, 1, 3\rbrace) = 1 + 2+ 2 = 5
$$

Primero, hay que fijar algo trivial: que, dado el mínimo natural que no se encuentre en nuestro conjunto $X$, entonces $mex(X)$ estará acotada por dicho natural.

Hay dos posibilidades acerca de nuestro $X$:

**Caso 1**: Que contenga todos los naturales de $0...n$, siendo $n$ el tamaño del conjunto $X$. Entonces, como $mex(Y)$ siempre estará acotada por el primer número que no esté en un conjunto $Y$, la idea para maximizar la suma es siempre agarrar el mínimo numero del conjunto $X$ para que $mex(Y)$ no pueda estar acotado por un número que no sea el máximo elemento (es decir, el último agregado a mi solución). Entonces, dada la solución parcial $Y_i \subseteq X_i$:

- si le agrego un número $x$ tal que $x \neq Min(X)$, entonces $mex(Y_{i+1}) = Max(Y_i) + 1 < x$

- si le agrego un número $x$ tal que $x = Min(X)$, entonces $mex(Y_{i+1}) = Max(Y_i) + 1 = x$

**Caso 2**: Que no contenga un número de $0...n$. Entonces, si el mínimo número no contenido es $x$, podremos realizar el caso 1 para los números del $0...x$. Luego, no importará el orden en que sigamos tomando números restantes, pues nunca obtendremos un resultado mayor al aplicar nuestra función $mex$ a la solución actual. En este caso, el resultado será

$$
f_{mex}(X)= caso1 (\lbrace 0,...,x \rbrace) + m \times x
$$

siendo $m$ la cantidad de números que hay mayores que $x$ en $X$
