## Ejercicio 18

A ojo se ve fácilmente que una solución es ir tomando los distintos subconjuntos de manera ordenada para obtener la mayor sumatoria de $mex(C)$. Llamo $f_{mex}$ a la función propuesta por el ejercicio.Por ejemplo, en el vector $\lbrace3, 0, 1\rbrace$, si lo ordenamos como $\lbrace 1, 0, 3\rbrace$, vamos a obtener:

$$
f_{mex}(\lbrace 1, 0, 3\rbrace) =\\  
mex(\lbrace 1 \rbrace) + mex(\lbrace 1, 0\rbrace) + mex(\lbrace 1, 0, 3\rbrace) = 0 + 2+ 2 = 4
$$

Mientras que si lo ordenamos crecientemente:

$$
f_{mex}(\lbrace 0, 1, 3\rbrace) =\\  
mex(\lbrace 0 \rbrace) + mex(\lbrace 0, 1\rbrace) + mex(\lbrace 0, 1, 3\rbrace) = 1 + 2+ 2 = 5
$$