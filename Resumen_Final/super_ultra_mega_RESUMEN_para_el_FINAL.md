# Resumen para final de Algoritmos y Estructuras de Datos III

## Complejidad computacional: Las reglas del juego

Suponemos una **Máquina RAM** (random access memory).
1. La memoria está dada por una sucesión de celdas numeradas. Cada celda puede almacenar un valor de $b$ bits.
2. Supondremos habitualmente que el tamaño $b$ en bits de cada celda está fijo, y suponemos que todos los datos individuales que maneja el algoritmo se pueden almacenar con $b$ bits.
3. Se tiene un programa imperativo no almacenado en memoria, compuesto por asignaciones y las estructuras de control habituales.
4. Las asignaciones pueden acceder a celdas de memoria y realizar las operaciones estándar sobre los tipos de datos primitivos habituales.  

Cada instrucción tiene un tiempo de ejecución asociado.
1. El acceso a cualquier celda de memoria, tanto para lectura como para escritura, es $O(1)$.
2. Las asignaciones y el manejo de las estructuras de control se realiza en $O(1)$.
3. Las operaciones entre valores lógicos son $O(1)$.  

Las operaciones entre enteros/reales dependen de $b$:
1. Las sumas y restas son $O(b)$.
2. Las multiplicaciones y divisiones son $O(b\ log\ b)$.  


$\Rightarrow$ Si $b$ está fijo, estas últimas operaciones son O(1). En cambio, si no se puede suponer esto, entonces hay que contemplar que el costo de estas operaciones depende de $b$.

### Poblemas de Optimización y Brute Force

Un problema de optimización consiste en encontrar la mejor
solución dentro de un conjunto:  
$$z\ast = máx_{x∈S} f (x)$$
o lo mismo pero con $mín$.  

La función $f: S \rightarrow \mathbb{R}$ se denomina función objetivo del problema, donde el conjunto $S$ es la región factible y los elementos $x \in S$ se llaman soluciones factibles.  

El valor $z \ast \in \mathbb{R}$ es el valor óptimo del problema, y cualquier solución factible $x\ast \in S$ tal que $f(x\ast) = z\ast$ se llama un óptimo del problema.  

Un problema de **optimización combinatoria** es un problema de optimización cuya región factible es un conjunto definido por consideraciones combinatorias, aquella rama de las matemáticas que estudia la construcción, enumeración y existencia de configuraciones de objetos finitas.  

Un algoritmo de **Fuerza Bruta** para un problema de optimización combinatoria consiste en generar TODAS las soluciones factibles y quedarse con la óptima.
1. Son algoritmos de búsqueda exhaustiva, o *generate and test*
2. Es una técnica trivial, pero muy general
3. Es fácil de implementar, y genera una solución exacta
4. Su solución tiene una complejidad computacional **exponencial**

## Backtracking

Idea: Recorrer sistemáticamente todas las posibles configuraciones del espacio de soluciones de un problema computacional, eliminando las configuraciones parciales que no puedan completarse a una solución.  
- Habitualmente, utiliza un vector $a = (a_1, a_2, ..., a_n)$ para representar una solución candidata, donde cada $a_i$ pertenece a un dominio/conjunto ordenado y finito $A_i$.  
- El espacio de soluciones es el producto cartesiano $A_1 × A_2 ×... × An$.

Se puede pensar este espacio como un árbol dirigido, donde cada vértice representa una solución parcial, y un vértice $x$ es hijo de $y$ si la solución parcial $x$ se puede extender desde la solución parcial $y$.   

Permite descartar configuraciones antes de explorarlas (podar el árbol).
```
algoritmo BT(a, k)
    si a es solución entonces
        procesar(a)
        retornar
    sino
        para cada a' ∈ Sucesores(a, k)
            BT(a0, k + 1)
        fin para
    fin si
    retornar
```

Un ejemplo es el *knapsack problem*, o problema de la mochila, un problema de optimización combinatoria que busca la mejor combinación de objetos con diferentes pesos y valores para incluir en una mochila, de manera que el valor total sea máximo y el peso no exceda la capacidad máxima de la mochila.  pod

El backtracking es eficiente para este problema porque evita explorar combinaciones que claramente no conducirán a la solución óptima, reduciendo así el espacio de búsqueda, como por ejemplo evaluar soluciones que se exceden del peso permitido. 

## Dynamic Programming

Se divide el problema principal en pequeños subproblemas que se resuelven recursivamente.  

Ejemplo de cálculo de coeficientes binomiales
$$
\
\binom{n}{k} = \frac{n!}{k!(n-k)!}
\
$$


No es buena idea computar esta definición directamente (¿por qué?).  

Teorema para el cálculo de coeficientes binomiales
$$
\
\binom{n}{k} =
\begin{cases}
1 & \text{si } k = 0 \text{ o } k = n, \\
\binom{n-1}{k-1} + \binom{n-1}{k} & \text{si } 0 < k < n.
\end{cases}
\   
$$

### Superposición de estados
Se da cuando el árbol de llamadas recursivas resuelve el mismo problema más de una vez.  

Alternativamente, podemos decir que se realizan muchas veces llamadas a la función recursiva con los mismos parámetros.  

Un algoritmo de programación dinámica entonces evita estas repeticiones con alguno de sus esquemas: *top-down* o *bottom-up*  


1. **Enfoque top-down**. Se implementa recursivamente, pero se guarda el resultado de cada llamada recursiva en una estructura de datos (_**memoización**_). Si una llamada recursiva se repite, se toma el resultado de esta estructura.  

2. **Enfoque bottom-up**. Resolvemos primero los subproblemas más pequeños y guardamos (habitualmente en una tabla) todos los resultados.

#### Ejemplo de coeficientes binomiales con DP:

```
algoritmo combinatorio(n, k)
  entrada: dos enteros n y k
  salida: nCk

  para i = 1 hasta n hacer
        A[i][0] ← 1
  fin para

  para j = 0 hasta k hacer
        A[j][j] ← 1
  fin para

  para i = 2 hasta n hacer
        para j = 1 hasta min(i - 1, k) hacer
              A[i][j] ← A[i - 1][j - 1] + A[i - 1][j]
        fin para
  fin para

  retornar A[n][k]
```

Por definición:

- Complejidad $O(n)$. _¡Ojo! el tamaño de entrada es $O(\log n!)$_.
    - Inconvenientes: inestabilidad numérica y/o manejo de enteros muy grandes.
    - Función recursiva:

- Complejidad $\Omega\left( {n \choose k} \right)$.

- Aplicando programación dinámica (bottom-up):
    - Complejidad $O(nk)$.
    - Espacio $O(k)$: sólo necesitamos almacenar la fila anterior de la que estamos calculando.
    - Se podría mejorar un poco sin cambiar la complejidad aprovechando que $({n \choose k} = {n \choose n-k})$.

#### Knapsack problem con DP

El problema de la mochila con un enfoque recursivo tiene una complejidad de $O(2^N)$, siendo $N$ el número de elementos.  

Al aplicar programación dinámica, la complejidad se reduce a $O(NW)$, donde $W$ es la capacidad de la mochila. Con optimizaciones de espacio, la complejidad de espacio de DP puede ser solo $O(W)$. Esto convierte un problema exponencial en uno manejable y polinómico.

#### Subsecuencia común más larga

```
SCML(A, B)
Entrada: A, B secuencias
Salida: Longitud de la SCML entre A y B

    l[0][0] ← 0
    para i = 1 hasta longitud(A) hacer
        l[i][0] ← 0
    fin para
    para j = 1 hasta longitud(B) hacer
        l[0][j] ← 0
    fin para
    para i = 1 hasta longitud(A) hacer
        para j = 1 hasta longitud(B) hacer
            si A[i] = B[j] entonces
                l[i][j] ← l[i-1][j-1] + 1
            sino
                l[i][j] ← max{l[i-1][j], l[i][j-1]}
            fin si
        fin para
    fin para
    retornar l[longitud(A)][longitud(B)]
```

Este algoritmo es $O(n^2)$ y requiere $O(n^2)$ espacio para almacenar $l$.

### Heurísticas
  
Una **heurística** es un procedimiento computacional que intenta obtener soluciones de buena calidad para un problema, intentando que su comportamiento sea lo más preciso posible,  

Por ejemplo, en un problema de optimización, una heurística obtiene una solución con un valor cercano o igual al óptimo.  

Decimos que $A$ es un algoritmo $\epsilon \text{-aproximado}$, con $\epsilon > 0$, para un problema, si:

$$
\left| \frac{X_A - X_{opt}}{X_{opt}} \right| \leq \epsilon
$$  

Hay otra forma de definir el factor/ratio de aproximación en la literatura. APX (approximable) y PTAS (polynomial-time approximation scheme) son 2 clases de Teoría de Complejidad Computacional relacionadas.  

## Algoritmos Greedy

Idea: construir una solución seleccionando en cada paso la mejor alternativa, sin considerar sus implicaciones

- Proporcionan heurísticas sencillas para problemas de optimización
- Permiten construir soluciones razonables en tiempos eficientes
- Aunque a veces pueden llevar a soluciones no optimas...

#### Knapsack problem con greedy

Un ejemplo es el knapsack problem en su versión fraccional, que puede ser resuelto utilizando un algoritmo greedy basado en el coeficiente beneficio/peso, eligiendo primero aquellos con el mayor valor de coeficiente.  

Es importante notar que este enfoque solo es aplicable para la versión fraccional del problema de la mochila, donde se permite tomar fracciones de un objeto. En el caso del problema de la mochila 0/1, donde los objetos deben ser tomados enteros o no ser tomados en absoluto, un algoritmo greedy basado en el coeficiente beneficio/peso no garantiza encontrar la solución óptima1.

#### Problema del cambio en monedas

Una solución greedy sería centrarse en elegir las monedas más grandes para llegar a la solución óptima, la menor cantidad de monedas necesarias para llegar al cambio solicitado. Es goloso pues en cada paso selecciona la moneda de mayor denominación posible.

#### Problema de espera total en un sistema

El objetivo es, dado un conjunto de clientes que tienen un tiempo de atención asocidado, determinar en qué orden se deben atender los clientes para minimizar la suma de los tiempos de espera de los clientes. Una solución greedy sería, en cada paso, atender al cliente pendiente que tenga menor tiempo de espera.


## Teoría de Grafos

Un **grafo** modela relaciones entre nodos o vértices, mediante ejes o aristas, generando una estructura flexible e intuitiva de representar una gran variedad de problemas, representables mediante una red.

- Circuito Euleriano: circuito que pasa exactamente una vez por cada arista
- Circuito Hamiltoniano: circuito que pasa exactamente una vez por cada vértice

### Algunas definiciones y teoremas

1. **Teorema:** Dado un grafo $G = (V, E)$, la suma de los grados de sus vértices es igual a 2 veces el número de aristas. 
2. **Def:** Un grafo $G = (V, E)$ se dice bipartito  si existen dos subconjuntos de vértices $V_1,  V_2$ tal que
   - $V = V_1 \cup V_2$
   - $V_1 \cap V_2 = \empty$
   - $\forall e = (u,v) \in E, u \in V_1 \land v \in V_2$
3. **Teorema:** Un grafo es bipartito $\iff$ no tiene ciclos de longitud impar.
4. **Def:** Dados $G=(V,E)$ y $G’=(V’,E’)$ son isomorfos si existe una función biyectiva $f: V \rightarrow V$, tal que para todo $u,v \in V, (u,v) \in E \iff (f(u),f(v)) \in E’$, siendo $f$ función de isomorfismo.  
Si dos grafos son isomorfos, entonces: 
    - tienen el mismo número de vértices
    - tienen el mismo número de aristas
    - tienen el mismo número de vértices de grado k (la misma distribución de grado)
    - tienen el mismo número de componentes conexas
    - tienen el mismo número de caminos simples de longitud k.

### Representación de grafos

1. **Lista de adyacencia:** estructura de datos que representa las relaciones entre los nodos en un grafo. Para cada nodo, se mantiene una lista de todos los nodos a los que es adyacente. Es eficiente en términos de memoria cuando el grafo es ralo, ya que solo almacena las conexiones existentes.  

2. **Matriz de adyacencia:** matriz cuadrada donde las filas y columnas representan los nodos del grafo. Los elementos de la matriz indican si hay una conexión directa entre los pares de nodos. Es útil para grafos densos y para operaciones matemáticas sobre el grafo, pero puede ser ineficiente en memoria para grafos con pocas aristas debido a que almacena información de conexiones incluso cuando no existen.

## Recorrido en grafos

Objetivos:
- Encontrar caminos (mínimos)
    - Medir distancias.
    - Estimar el diámetro del grafo: Camino mínimo más largo.
- Encontrar todos los nodos alcanzables desde una fuente.
- Encontrar ciclos.
- Ordenar nodos
- Encontrar Componentes Fuertemente Conexas (c.f.c.)
- Encontrar el Árbol Generador Mínimo 

### Breadth First Search (BFS)

- **Objetivo:** Visitar todos los nodos accesibles (alcanzables) desde una fuente s (source):
    - Nos permite computar la distancia mínima hasta esos nodos.
    - Generar un árbol a través de los caminos generados.

- Complejidad de tiempo: $Θ(V+E)$

- **Idea:** Visitar los vecinos de s, luego los vecinos de los vecinos, y así sucesivamente (por capas):
    - En 0 movidas ⟶ s
    - En 1 movida ⟶ Adj[s]
    - En 2 movidas ⟶ ...
    - ...

- Contiene las ideas generales de otros algoritmos como:
  - Prim (Árbol Generador Mínimo)
  - Dijkstra (Camino Mínimo)

```py
BFS(s, Adj):
    level = {s: 0}
    parent = {s: None}
    i = 1
    frontier = [s]  # level i-1
    while frontier: # para cada nodo
        next = []  # level i
        for u in frontier:
            for v in Adj[u]: # visita solo una vez cada vecino
                if v not in level:
                    level[v] = i
                    parent[v] = u
                    next.append(v)
        frontier = next
        i += 1
```

### Depth First Search (DFS)

- **Objetivo:** Visitar todos los nodos.
  - Generar un bosque a través de los caminos generados.
  - Clasificar aristas.
  - Detectar ciclos.
  - Ordenar secuencias de estados (Algoritmo topological sort).
  - Detectar componentes fuertemente conexas (Algoritmo de Kosaraju).

- Complejidad de tiempo: $\Theta(V+E)$

- **Idea:** Empiezo por un nodo y voy visitando a un vecino, a un vecino de este vecino, etc… hasta agotar (en profundidad); luego empiezo por otro; y así siguiendo...
    - Se arma de forma recursiva y con backtracking hasta donde encuentre un nuevo camino para ir en profundidad.
    - Es importante guardar registro para no volver a explorar nodos ya visitados.

```py
DFS-Visit(s, Adj):
    visited[s] = true
    previsit(s)
    for v in Adj[s]:
        if not visited[v]:
            parent[v] = s
            DFS-Visit(v, Adj)
    postvisit(s)

DFS(G):
    for each u in G.V:
        visited[u] = false
        parent[u] = None
    for each u in G.V:  # cada vértice se explora solo una vez
        if not visited[u]:
            DFS-Visit(u, G.Adj) # cada vecindario se explora solo una vez
```

#### Detección de ciclos con DFS

- Dado un grafo  $G = (V,E)$,  
  $G$ tiene **ciclo** $\iff$ el **árbol DFS** tiene una **_backedge_**

- Dado un digrafo  $D = (V,E)$,  
  $D$ tiene **ciclo** $\iff$ el **bosque DFS** tiene una _**backward edge**_


### Topological Sort

Dado un DAG (_Directed Acyclic Graph_), podemos ordenar los vértices para que todas las aristas apunten de menor a mayor orden

- **Orden Lineal**: Organiza los vértices en una línea tal que para cada arista dirigida $(u \rightarrow v)$, $u$ viene antes que $v$.  
- **Dependencias**: Se usa para programar tareas respetando sus dependencias.
- Existen algoritmos para realizar un ordenamiento topológico en tiempo lineal. Una idea sería:
  - Elegir un nodo no visitado como fuente
  - Hacer un DFS a partir de este
  - En la recursión del DFS, agregar el nodo en el sorting topológico en orden reverso
- No hay un único orden topológico pues depende de qué nodo arranquemos.

### Componentes Fuertemente Conexas y algoritmo de Kosaraju 

Un componente fuertemente conectado es un subgrafo en el que para cada par de vértices $u$ y $v$, existe un camino de $u$ a $v$ y un camino de $v$ a $u$. Es decir, todos los vértices del componente están interconectados entre sí.

- Aplicaciones: Descomponer en c.f.c. es el punto de
partida (requisito) de muchos algoritmos, porque permite procesar cada componente de manera independiente.

El **algoritmo de Kosaraju** es un algoritmo de tiempo lineal utilizado para encontrar los c.f.c. de un digrafo. 

El algoritmo de Kosaraju funciona de la siguiente manera:

1. Marca todos los vértices como no visitados.
2. Realiza DFS para cada vértice no visitado y al terminar, añade el vértice al principio de una lista.
3. Traspone el grafo, invirtiendo aristas.
4. Realiza un DFS en el grafo transpuesto, comenzando por los últimos vértices de la lista obtenida en el paso 2.
5. Cada árbol del bosque resultante de la DFS representa un componente fuertemente conectado del grafo original.  

Esto funciona ya que $G$ y $G^T$ tienen las mismas c.f.c.


## Árboles y más

Un **árbol** es un grafo conexo acíclico.
- Si se le agrega una arista, se forma un ciclo
- Tiene $n-1$ aristas, siendo $n$ la cantidad de nodos
- Si se le saca cualquier arista, se desconecta.
- Existe un único camino entre cada par de nodos

Un **bosque** es un grafo sin caminos.  
Sea $G = (V, X)$ un bosque con $c$ componentes conexas. Entonces $m = n − c$.

Un **árbol enraizado** tiene:
- Raíz: Algún vértice elegido
- Hojas: $u$ tq $d(u) = 1$
- Vértices internos: Ni hojas ni raíces
- Altura $h$: De la raíz a la hoja más lejana.
- Árbol $m$-ario: Donde m es el número máximo de hijos un nodo (si
todos los vértices $v$ tienen $d(v) ≤ m+1$ y la raíz $r$ tiene $d(r) ≤ m)$.
- Nivel: “Altura” de un vértice o distancia a la raíz.
- Árbol balanceado: Todas sus hojas están a nivel $h$
1. $T$ es $m$-ario de altura $h$ $\Rightarrow$ tiene a lo sumo $l=m^h$ hojas
2. $T$ es $m$-ario con $l$ hojas ⇒ tiene $h ≥  ⎡log_m
(l)⎤$ hojas

## Árbol Generador

En muchas aplicaciones se quiere conectar $n$ puntos (ciudades, centrales, servidores) mediante una red. Hay ciertas alternativas de enlaces posibles que se pueden construir, representadas mediante un grafo. Lo primero que tenemos que requerir es que este grafo sea conexo, de lo contrario no vamos a poder conectar todas las ciudades. Si queremos diseñar la red con la menor cantidad de enlaces posibles, vamos a necesitar elegir $n−1$ conexiones de las posibles, formando un árbol.  

Un **árbol generador** de un grafo $G$ es un subgrafo que tiene el mismo conjunto de vértices y es un árbol.
- Todo $G$ conexo tiene al menos un AG.
- Si $G$ conexo tiene un sólo AG, entonces es un árbol.

Un **árbol generador mínimo** (AGM).  
Dado un grafo $G=(V, E, w)$ con $w: E \rightarrow R$:
- El peso del AGM $T$ es: $w(T) = \sum_{e\in E_T} w(e)...$, es decir, la suma de los pesos de las aristas del árbol generador
- AGM es el AG para el cual $w(T)$ es mínima.
- Para los grafos no pesados, todo AG es AGM
- Puede haber varios AGM

Los dos algoritmos vistos para obtener un AGM son **Prims** y **Kruskal**

### Prims

El algoritmo de Prim, dado un grafo $G = (V, E)$, construye incrementalmente dos conjuntos, uno de vértices $V_T$ (inicializado con un vértice cualquiera) y otro de aristas $E_T$, que comienza vacío.  

En cada iteración, se agrega un elemento a cada uno de estos conjuntos. Cuando $V_T = V$, el algoritmo termina y las aristas de $E_T$ definen un AGM de $G$.  

En cada paso, se selecciona la arista de menor costo entre las que tienen un extremo en $V_T$ y el otro en $V\ \backslash \  V_T$ (las de la "frontera"). Esta es una elección golosa: de un conjunto de aristas candidatas, elige la mejor arista (en este caso la de menor costo). Esta arista es agregada a $E_T$ y el extremo a $V_T$.  

Se puede implementar con complejidad $O(n^2)$ o bien $O(m\ log\ n)$.  

Una implementación en $O(m\ log\ n)$ es la siguiente:

```py
PRIM ( r , G ) :
    for u in V:         # O(n)
        u.key = Inf
        u.parent = None
    r.key = 0
    Q = array_to_heap(V) # O(n) armo min-heap

    while not Q.empty :
        u = EXTRACT-MIN(Q)  # O(log n) entre el while y el extract
        for v in Adj[u] :   # O(m)
            if (v in Q) AND (w(u,v) < v.key):
                v.parent = u
                v.key = w(u,v)
                DECREASE-KEY(Q, v, w(u,v)) # O(log n)

```

Donde la `key` de cada vértice se refiere al menor peso conocido desde cualquier vértice en el árbol generador actual hasta dicho vértice

#### Correctitud de Prims

En cada iteración de $while$, se extrae un nodo $u$ de $Q$ y quedan congelado tanto el valor de $u.parent$ como el de $u.key$. Luego de dos iteraciones, $u.parent$ es un vecino de $u$ fuera $Q$, llamamos $e_i = (u, u.parent)$ y $u.key = w(e_i)$. Por lo tanto, cuando termina el algoritmo, se obtiene $T\ast=(e_2, ..., e_n)$.  

Es fácil de ver por inducción que después de completar la iteración $i$, las primeras $i-1$ aristas de $T\ast$ conectan a todos los nodos fuera de $Q$. Entonces $T\ast$ es AG de $G$.  

Ahora probemos que $T\ast$ es AGM de $G$. Supongamos que no lo es, entonces dado cualquier AGM $T$ de $G$, definimos $dif(T\ast, T) = min(j\ tq\ e_j \in T\ast \backslash T)$.  

Sea $P_{ab}$ el camino simple que une $a$ con $b$ en $T$. Sin pérdida de generalidad, al comienzo de la iteración $k$ tenemos $a \in Q, b \notin Q, a.parent = b, a.key = w(e_k)$ y más adelante se elige $u=a$.  

Podemos afirmar que existe una arista $f=(x,y) \in P_{ab}$ tq $x \in Q, y \notin Q$ ya que $a \in Q, b \notin Q$.  

Consideramos ahora $T' = T + e_k - f$, que es AG porque $P_{ab} + e_k$ es un ciclo y $f$ está en ese ciclo.  

Por otro lado, $w(f) \geq x.key \geq a.key = w(e_k)$ pues se había considerado $w(f)$ para determinar el valor de $x.key$ cuado $y$ fue sacado de $Q$ y se iba a elegir $a$ en la interación $k$ en lugar de $x$.   

Por lo tanto, $w(T') \leq w(T)$, pero como $T$ es AGM, entonces $w(T') = w(T)$. $T'$ sería otro AGM de $G$.  

$T'$ ahora contiene a $e_k$ pero también conserva las aristas $e_2, ..., e_{k-1}$, ya que tienen sus ambos extremos fuera de $Q$. Entonces $dif(T\ast, T') > k = dif(T\ast, T)$, contradicción.


### Kruskal

La idea de este algoritmo es ordenar las aristas del grafo de forma creciente según su peso, y en cada paso elegir la siguiente arista que no forme circuito con las aristas ya elegidas. El algoritmo para cuando selecciona $n − 1$ aristas (siendo $n$ la cantidad de vértices del grafo). También es un algoritmo goloso.  

Al comenzar el algoritmo, cuando todavía no se seleccionó arista alguna, cada vértice del grafo forma una componente conexa distinta (es un **bosque** de vértices aislados). En la primera iteración, los dos vértices extremos de la arista seleccionada van a pasar a formar una única componente conexa del nuevo bosque. Así, en cada iteración, si se elige la arista $(u, w)$, se unen las componentes conexas de $u$ y la de $w$. En cada iteración el bosque obtenido tiene una componente conexa menos que el anterior. El algoritmo termina cuando el bosque pasa a ser un árbol, es decir, conexo.

Se puede implementar con complejidad $O(n^2)$ o bien $O(m\ log\ n)$.  

Una implementación en $O(m\ log\ n)$ es la siguiente:

```py
KRUSKAL ( G ) :
    A = Ø
    for u in G.V:       # O(n)
        MAKE-SET( u ) # O(1)

    ORDENAR E de menor a mayor por w(e) # O(m log n)
    for e in G.E: # en orden de w, esto tiene una complejidad rara O(m alpha n)
        if FIND-SET(u) ≠ FIND-SET(v): # para e=(u,v)
            A = A ∪ {e}
            UNION(u,v)
```

#### Correctitud de Kruskal

La demostración de correctitud de Kruskal es muy similar a la de Prim. En cada iteración, el grafo que arma el algoritmo es un bosque, ya que no contiene circuitos. Al finalizar la última iteración, como se incorporaron $n−1$ aristas, pasa a ser árbol (sin circuitos y con $m−1$ aristas). Esto junto a la siguiente proposición demuestra la correctitud de Kruskal.

## Camino mínimo

Dado un grafo orientado $G=(V,E)$ con longitudes asociadas a sus aristas $(l : E \rightarrow R)$, la longitud (o peso o costo) de un camino es la suma de las longitudes de sus aristas.  

El problema del camino mínimo consiste en encontrar el camino de menor longitud...
- de un vértice a otro
- de un vértice a todos los demás
- entre todos los pares de vértices  


La distancia de $v$ a $w$, $d(v,w)$, es la longitud de un camino mínimo entre $v$ y $w$, $+ \inf$ si no existe ningún camino de $v$ a $w$, y $- \inf$ si existen caminos de $v$ a $w$ pero no uno mínimo (solo si existe un ciclo de longitud negativa).

> **Principio de optimalidad**
> Todo subcamino de un camino mínimo es un camino mínimo.


> **Árbol de caminos mínimos**
> Sea $G$ un grafo, $s$ un vértice de $G$. Sea $W$ el conjunto de vértices alcanzabes desde $s$ en $G$. Entonces existe un árbol orientado $T$ con raíz $s$ y $V(T) = W$, tal
que para todo $v \in W$, el camino de $s$ a $v$ en $T$ es un camino mínimo de $s$ a $v$ en $G$.  

(Tiene su debida demostración en el pdf)

### Algoritmo de Dikstra-Moore  

Es un algoritmo goloso que construye un árbol de caminos mínimos (un árbol enraizado en $s$ conteniendo un camino mínimo desde $s$ hacia todo otro vértice de $V$), comenzando con el vértice $s$ y agregando un vértice a este árbol en cada iteración. Es goloso porque en cada iteración agrega el vértice más cercano a $s$ de entre todos los que todavía no fueron agregados al árbol. Es decir, en la iteración $k$ agrega al árbol de caminos mínimos el $k$-ésimo vértice más cercano a $s$.  

Funciona cuando el grafo no tiene aristas negativas alcanzables desde $s$.  

- Input del algoritmo: grafo $G = (V, E)$, $V = \{1, . . . , n\}$, longitudes $\ell : E \rightarrow \mathbb{R}^{+}$, vértice $s$.

- Output del algoritmo: árbol de caminos mínimos desde el vértice $s$.

```py
DIJKSTRA(G, w, s)
    INITIALIZE-SINGLE-SOURCE(G, s)  # distancias de s a cada nodo
    S = ∅                           # conj de nodos con dist ya determinadas
    Q = G.V                         # cola de nodos a visitar
    for each vertex u in G.V
        INSERT(Q, u) 
    while Q ≠ ∅
        u = EXTRACT-MIN(Q)
        S = S ∪ {u} 
        for each vertex v in G.Adj[u]
            RELAX(u, v, w)
            if the call of RELAX decreased v.d
                DECREASE-KEY(Q, v, v.d) # si el camino es mas corto, atualizo distancia
```

Complejidad: $O(n^2)$ o $O(m\ log \n)$


### ALgoritmo de Bellman-Ford

Este algoritmo resuelve, utilizando la técnica de programación dinámica, el problema de camino mínimo con único origen $s$ a múltiples destinos, con arcos de peso tanto positivo como negativo.  

Para cada vértice $u$, en $π[u]$ va guardando una cota superior de la longitud de un camino mínimo desde $s$ a $u$, hasta obtener la verdadera longitud. El algoritmo considera sucesivamente caminos con más cantidad de arcos. Al terminar la $k$-ésima iteración, el algoritmo encuentra todos los caminos mínimos desde $s$ con, a lo sumo, $k$ arcos. En cada iteración se revisan todos los arcos, y, si agregar un arco $(w \rightarrow u)$ al camino hasta $w$ obtenido hasta el momento es más corto que ese camino, actualiza el valor del camino hasta $u$, $π(u)$, poniendo a $w$ como su predecesor. El algoritmo se detiene cuando en una iteración no es posible mejorar ningún camino. Como los caminos mínimos son simples, a lo sumo realiza $n-1$ iteraciones.

```py
BELLMAN-FORD(G, w, s)
    INITIALIZE-SINGLE-SOURCE(G, s)

    for i = 1 to |G.V| - 1
        for each edge (u, v) in G.E
            RELAX(u, v, w)

    for each edge (u, v) in G.E
        if v.d > u.d + w(u, v)
            return FALSE # hay ciclo de long negativa

    return TRUE

```

El **invariante** del algoritmo es $d_k​(i) =$ camino mínimo de 1 a $i$ que usa a lo sumo $k$ aristas, o $\inf$ si no existe tal camino.  

Si no hay ciclos negativos alcanzables desde $s$, hay camino mínimo con a lo sumo $n−1$ aristas. Entonces, con este invariante, el algoritmo funciona.  

Complejidad: $O(nm)$


### Algoritmo de Floyd-Warshall

El algoritmo de Floyd calcula el camino mínimo entre todo par de vértices de un digrafo pesado. Utiliza la técnica de programación dinámica.  

Al finalizar la iteración $k$ del algoritmo de Floyd, $d[i][j]$ es la longitud de los caminos mínimos desde $v_i$
a $v_j$ cuyos vértices intermedios son elementos de $\{v_1, . . . , vk\}$.

```py
FLOYD(G):
    dist = array(n, array(n))

    for k in range(n):
        for i in range(n):
            for j in range(n):
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])

    return dist
```


### Algoritmo de Dantzig  

En la iteración $k$, el algoritmo de Dantzig genera una matriz de $k \times k$ de caminos mínimos en el subgrafo inducido
por los vértices $\{v_1, . . . , v_k\}.


## Flujo en redes

- Datos de entrada:
  1. Un grafo dirigido $G = (N, A)$.
  2. Nodos $s,t \in N$ de origen y destino.
  3. Una función de capacidad $u : A → \mathbb{Z}_+$ asociada con los arcos.  

Algunas propiedades: 

1. **Ley de conservación de flujo**: Salvo $s$ y $t$, en cada nodo la cantidad de flujo que entra al nodo debe ser igual a la cantidad de flujo que sale del nodo. 
2. La cantidad $x_{ij}$​ enviada por el arco $ij \in A$ debe cumplir $0 \leq x_{ij} ​\leq u_{ij}$​.  

Si cumple 1 y 2, es un **flujo factible**  

El valor de un flujo es la cantidad de flujo neto que sale de $s$.  

### Problema de flujo máximo

El problema de flujo máximo consiste en, dada una red $N=(V,A)$ con capacidades en los arcos y dos vértices específicos, una fuente $s$, y un sumidero $t$, determinar el flujo factible de valor máximo $F$ que se puede enviar de $s$ a $t$.  

Un **corte** en la red $G=(N,A)$ es un subconjunto $S⊆N \backslash {t}$ tal que $s \in S$.  
Dados $S,T⊆N$, definimos $ST=\{ij∈A:i∈S y j∈T\}$.  

- Proposición 1: Dado un flujo definido en una red y un corte $S$, el valor del flujo será la suma de los flujos salientes de $S$, menos los flujos entrantes a $S$.  

La capacidad de un corte $S$ se define como la suma de las capacidades salientes de $S$.  

- Proposición 2: Si $x$ es un flujo con valor $F$ y $S$ es un corte en $N$, entonces $F \leq u(S)$.

> Corolario (**certificado de optimalidad**): Si $F$ es el valor de un flujo $x$ y $S$ un corte en $G$ tal que $F = u(S)$ entonces $x$ define un flujo máximo y $S$ un corte de capacidad mínima.