## Ejercicio 2

El algoritmo lo que hará es recorrer de atrás para adelante el arreglo de intervalos $(s_i, t_i)$ de largo $n$. Empezamos agarrando el primer elemento $s_n$ del primer intervalo. Luego, teniendo en cuenta este número, miramos $t_{n-1}$, es decir, el final del intervalo que sigue detrás; si $t_{n-1}$ es mayor que $s_n$, entonces no agarro ese intervalo, y sigo mirando en el resto. Cuando encuentro uno que no sea mayor o igual, entonces agarro su principio y repito este algoritmo. +

Es decir, cuando $t_{i-1} < s_i$, agrego $s_i$ a $P$. Si no se cumple esto, me sigo fijando con $t_{i-2} < s_{i-1}$, luego $t_{i-3} < s_{i-2}$, y  así hasta encontrar un caso que lo cumpla.
