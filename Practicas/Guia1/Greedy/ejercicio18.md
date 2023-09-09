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

**Caso 1**: Que contenga todos los naturales de $0...n$, siendo $n$ el tamaño del conjunto $X$. Entonces, como $mex(Y)$ siempre estará acotada por el primer número que no esté en un conjunto $Y$, la idea para maximizar la suma es siempre agarrar el mínimo numero del conjunto $X$ para que $mex(Y)$ no pueda estar acotado por un número que no sea el máximo elemento (es decir, el último agregado a mi solución).  

Mi inducción será sobre el que agarrar siempre el mínimo elemento para realizar el siguiente paso es mejor o igual solución que agarrar cualquier otro elemento.

$$
P(n) = \sum_{x = min(X_i)} mex(X_{i} \cup x) \geq \sum_{y \in X_i} mex(X_{i} \cup y)
$$

$$
P(1) \Rightarrow |X| = 1 \Rightarrow \text{agarrar mínimo o cualquier otro será lo mismo. Entonces lo cumple.}
$$

$$
P(2) \Rightarrow |X| = 2 \Rightarrow \text{en este caso ocurre:}
$$

$$
\sum_{x = min(X_i)} mex(X_{i} \cup x) = mex(min(X)) + mex(min(x) \cup max(x)) \geq \sum_{y \in X_i} mex(X_{i} \cup y) = mex(max(X)) + mex(max(x) \cup min(x))
$$  

Como son solo dos elementos, el que no es mínimo será el máximo.  

$$
\text{si } min(X) > 0 \text{ entonces ambas sumatorias darán igual por el caso trivial mencionado antes, y se cumple P(2). En caso contrario:}
$$

$$
mex(\lbrace 0 \rbrace) + mex(\lbrace 0 \cup max(X) \rbrace) > mex(\lbrace max(X) \rbrace) + mex(\lbrace max(X) \cup 0 \rbrace)
$$

$$
1 + mex(\lbrace 0 \cup max(X) \rbrace) > 0 + mex(\lbrace max(X) \cup 0 \rbrace)
$$

$$
1 > 0 
$$

Entonces como cumple con $P(1)$ y $P(2)$ ahora quiero ver que se cumple para $P(n+1)$ según mi hipótesis inductiva $\text{HI: cumple } P(n) \text{, es decir, para el paso } i+1$  

$$
\sum_{x = min(X_{i+1})} mex(X_{i+1} \cup x) \geq \sum_{y \in X_{i+1}} mex(X_{i+1} \cup y)
$$  

Si $y=x$ entonces cumple el caso porque son iguales. Si no:  

$$
\sum_{x = min(X_{i+1})} mex(X_{i+1} \cup x) > \sum_{y \in X_{i+1}} mex(X_{i+1} \cup y)
$$  

$$
\sum_{x = min(X_{i+1})} mex(X_{i+1} \cup x) = \sum_{x = min(X_{i})} mex(X_{i} \cup x) + mex(X_{i}\cup min(X_{i+1})) \geq ^{HI} \sum_{y \in X_{i}} mex(X_{i} \cup y) + mex(X_{i}\cup min(X_{i+1}))
$$  

$$
\sum_{y \in X_{i}} mex(X_{i} \cup y) + mex(X_{i}\cup min(X_{i+1})) \geq ^{QVQ}  \sum_{y \in X_i} mex(X_{i} \cup y) + mex(X_{i}\cup y) = \sum_{y \in X_{i+1}} mex(X_{i+1} \cup y)
$$

Me quito lo que tengo igual de cada lado:

$$
mex(X_{i}\cup min(X_{i+1})) \geq  mex(X_{i}\cup y)
$$

Entonces, como dijimos antes, vale para $x=y$ ya que tenemos un mayor o igual. Si no son iguales, entonces, como $y$ no es el mínimo, tiene que ser mayor. Esto implica que el término de la derecha se estará saltando un número al agregar el siguiente (ya que analizamos el caso en que el arreglo es una "escalera"), dejando así un resultado $mex$ que no será óptimo. Dicho de otra forma:  

- si le agrego un número $x$ tal que $x = min(X)$, entonces $mex(X_{i+1}) = max(X_i) + 1 = x$ (término de la **izquierda** en la desigualdad)

- si le agrego un número $y$ tal que $y \neq min(X)$, entonces $mex(X_{i+1}) = max(X_i) + 1 < x$ (término de la **derecha** en la desigualdad)

Por lo tanto, queda probado para $P(n+1)$

**Caso 2**: Que no contenga un número de $0...n$. Entonces, si el mínimo número no contenido es $x$, podremos realizar el caso 1 para los números del $0...x$. Luego, no importará el orden en que sigamos tomando números restantes, pues nunca obtendremos un resultado mayor al aplicar nuestra función $mex$ a la solución actual. En este caso, el resultado será

$$
f_{mex}(X)= caso1 (\lbrace 0,...,x \rbrace) + m \times x
$$

siendo $m$ la cantidad de números que hay mayores que $x$ en $X$
