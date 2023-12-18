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

#### Knapsack problem

Un ejemplo es el *knapsack problem*, o problema de la mochila, un problema de optimización combinatoria que busca la mejor combinación de objetos con diferentes pesos y valores para incluir en una mochila, de manera que el valor total sea máximo y el peso no exceda la capacidad máxima de la mochila.  pod

El backtracking es eficiente para este problema porque evita explorar combinaciones que claramente no conducirán a la solución óptima, reduciendo así el espacio de búsqueda, como por ejemplo evaluar soluciones que se exceden del peso permitido. 

## Dynamic Programming

Se divide el problema principal en pequeños subproblemas que se resuelven recursivamente.  

Ejemplo de cálculo de coeficientes binomiales:  

$$
\
\binom{n}{k} = \frac{n!}{k!(n-k)!}
\
$$


No es buena idea computar esta definición directamente (¿por qué?).  

Teorema para el cálculo de coeficientes binomiales:  

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

- Complejidad $O(n)$. ¡Ojo! el tamaño de entrada es $O(log\ n!)$.
    - Inconvenientes: inestabilidad numérica y/o manejo de enteros muy grandes.
    - Función recursiva

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

> Teorema: El algoritmo goloso por menor tiempo de atención proporciona una solución óptima del problema de minimizar el tiempo total de espera en un sistema.

## Teoría de Grafos

Un **grafo** modela relaciones entre nodos o vértices, mediante ejes o aristas, generando una estructura flexible e intuitiva de representar una gran variedad de problemas, representables mediante una red.

- Circuito Euleriano: circuito que pasa exactamente una vez por cada arista
- Circuito Hamiltoniano: circuito que pasa exactamente una vez por cada vértice

### Algunas definiciones y teoremas

1. **Teorema:** Dado un grafo $G = (V, E)$, la suma de los grados de sus vértices es igual a 2 veces el número de aristas. 
2. **Def:** Un grafo $G = (V, E)$ se dice bipartito  si existen dos subconjuntos de vértices $V_1,  V_2$ tal que
   - $V = V_1 \cup V_2$
   - $V_1 \cap V_2 = \emptyset$
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
    Q = min_heap {}
    for u in V:         # O(n)
        u.key = Inf
        u.parent = None
        Q.insert(u)
        
    r.key = 0

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

Sea $G$ un grafo conexo y ponderado.  

En toda iteración del algoritmo de Prim, se debe encontrar una arista que conecte un nodo del subgrafo a otro nodo fuera del subgrafo.  

Ya que $G$ es conexo, siempre habrá un camino para todo nodo.  

La salida $Y$ del algoritmo de Prim es un árbol porque las aristas y los nodos agregados a $Y$ están conectados.  

Sea $Y_{1}$ el árbol recubridor mínimo de $G$.  

Si $Y_{1} = Y \Rightarrow Y$ es el árbol recubridor mínimo.  

Si no, sea $e=(u,v)$ la primera arista agregada durante la construcción de $Y$, que no está en $Y_{1}$ y sea $V'$ el conjunto de nodos conectados por las aristas agregadas antes que $e$ (las del subgrafo formado hasta el momento). Entonces un extremo de $e$ está en $V'$ y el otro no. Ya que $Y_{1}$ es el árbol recubridor mínimo de $G$, existe un camino en $Y_{1}$ que une $u$ y $v$, ya que es un árbol generador de un grafo conexo. Si vemos dicho camino de $Y_1$, existe una arista $f$ uniendo un nodo en $V'$ a uno que no está en $V'$. En la iteración que $e$ se agrega a $Y$, $f$ también se podría haber agregado y se hubiese agregado en vez de $e$ si su peso fuera menor que el de $e$. Ya que $f$ no se agregó se concluye:  

$$
P(f) \geq P(e)
$$

Sea $Y_{2}$ el grafo obtenido al remover $f$ y agregando $e$ a $Y_{1}$. Es fácil mostrar que $Y_{2}$ conexo tiene la misma cantidad de aristas que $Y_{1}$, y el peso total de sus aristas no es mayor que el de $Y_{1}$, entonces también es un árbol recubridor mínimo de $G$ y contiene a $e$ y todas las aristas agregadas anteriormente durante la construcción de $V$. Si se repiten los pasos mencionados anteriormente, eventualmente se obtendrá el árbol recubridor mínimo de $G$ que es igual a $Y$.  

Esto demuestra que $Y$ es el árbol recubridor mínimo de $G$.  



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

Complejidad: $O(n^2)$ o $O(m\ log\ n)$


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
por los vértices $\{v_1, . . . , v_k\}$.


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


Más formalmente, para una red $G=(N,A)$ con fuente $s$ y sumidero $t$, y un flujo factible $f$, para todo $v,w \in N$ definimos su **capacidad residual** $u_f(v,w)$ como:  

$$
u_f(v,w) = 
\begin{cases} 
u(v,w) - f(v,w) & \text{si } (v,w) \in A \\
f(v, u) & \text{si } (w,v) \in A \\
0 & \text{de lo contrario}
\end{cases}
$$  

La **red residual** $R(G,f)$ es la red inducida por las capacidades $u_f$ del grafo $G$ con flujo factible $f$.  

Un **camino de aumento** es un camino orientado de $s$ a $t$ en $R(G,x)$.  

- Teorema: Sea $f$ un flujo definido sobre una red N. Entonces $f$ es un flujo máximo $\iff$ no existe camino de aumento en $R(G, f)$.  


- Teorema (**max flow-min cut**): Dada una red $N$, el valor del flujo máximo es igual a la capacidad del corte mínimo.

### Método Ford-Fulkerson

```
ALGORITMO FORD-FULKERSON(G, s, t)
    para cada arista (u, v) en G.E
        (u, v).f = 0
    mientras exista un camino p desde s hasta t en la red residual Gf
        cf(p) = min {cf(u, v) | (u, v) está en p}
        para cada arista (u, v) en p
            si (u, v) está en G.E
                (u, v).f = (u, v).f + cf(p)
            sino
                (v, u).f = (v, u).f - cf(p)
    return f
```

- **Teorema**: Si las capacidades de los arcos de la red son enteras, entonces el problema de flujo máximo tiene un flujo máximo entero.
- **Teorema**: Si los valores del flujo inicial y las capacidades de los arcos de la red son enteras, entonces el método de Ford y Fulkerson realiza a lo sumo $nU$ iteraciones, donde $U$ es una cota superior finita para el valor de las capacidades.
- Si las capacidades o el flujo inicial son números irracionales, el método de Ford y Fulkerson puede no parar (es decir, realizar un número infinito de pasos).  

Complejidad $O(nF)$, siendo $F$ el flujo máximo posible.

### Algoritmo de Edmond-Karp  

Sigue el método de Ford-Fulkerson, pero utilizando BFS para hallar el camino de aumento en cada iteración.  

Complejidad $O(nm^2)$

### Matching máximo en grafos bipartitos

- Un matching o correspondencia entre los vértices de $G$, es un conjunto $M ⊆ E$ de aristas de G tal que para todo $v ∈ V$, $v$ es incidente a lo sumo a una arista de $M$.

- El problema de matching máximo consiste en encontrar un matching de cardinal máximo entre todos los matchings de $G$.

- El problema de matching máximo es resoluble en tiempo polinomial para grafos en general (Edmonds, 1961–1965).

- Pero en el caso de grafo bipartitos, podemos enunciar un algoritmo más simple transformándolo en un problema de flujo máximo en una red.  

Dado el grafo bipartito $G = (V1 ∪ V2, E)$ definimos la siguiente red $N = (V', E')$:
- $V' = V1 \cup V2 \cup \{s,t\}$, con $s$ y $t$ dos vértices ficticios representando la fuente y el sumidero de la red.

- $E' = \{(i, j) : i \in V_1, j \in V_2, ij \in E\}
\cup \{(s, i) : i \in V_1\}
\cup \{(j,t) : j \in V_2\}$.

- $uij = 1$ para todo $ij \in E$.  


El cardinal del matching máximo de $G$ será igual al valor del flujo máximo en la red $N$.

## Complejidad computacional, P vs NP

Un **algoritmo eficiente** es un algoritmo de **complejidad polinomial**.  

Un problema está bien resuelto si se conocen algoritmos eficientes para resolverlo.  

El objetivo es clasificar los problemas según su complejidad.  

Una **instancia** de un problema es una especificación de sus parámetros. Un problema de decisión $\pi$ tiene asociado un conjunto $D_\pi$ de instancias y un subconjunto $Y_\pi ⊆ D_\pi$ de instancias cuya respuesta es SÍ.   

Dada una instancia $I$ del problema $\pi$:
- **Versión de evaluación**: Determinar el valor de una solución óptima de $\pi$ para $I$.
- **Versión de optimización**: Encontrar una solución óptima del problema $\pi$ para $I$ (de valor mínimo o máximo).
- **Versión de decisión**: Dado un número $k$, ¿existe una solución factible de $\pi$ para $I$ tal que $c(S) \leq k$ si el problema es de minimización (o $c(S) \geq k$ si el problema es de maximización)?
- **Versión de localización**: Dado un número $k$, determinar una solución factible de $\pi$ para $I$ tal que $c(S) \leq k$.  

Ejemplo: TSP  

Dado un grafo completo $G$ con longitudes asignadas a sus aristas:
- **Versión de evaluación**: Determinar el valor de una solución óptima, o sea la longitud de un circuito Hamiltoniano de $G$ de longitud mínima.
- **Versión de optimización**: Determinar un circuito Hamiltoniano de $G$ de longitud mínima.
- **Versión de decisión**: Dado un número $k$, ¿existe un circuito Hamiltoniano de $G$ de longitud menor o igual a $k$?
- **Versión de localización**: Dado un número $k$, determinar un circuito Hamiltoniano de $G$ de longitud menor o igual a $k$.


**Problemas intratables**: Un problema es intratable si no puede ser resuelto por algún algoritmo eficiente.  

Un problema puede ser intratable por distintos motivos:
- El problema requiere una respuesta de longitud exponencial (ejemplo: pedir todos los circuitos Hamiltonianos de longitud a lo sumo $k$).
- El problema es indecidible (ejemplo: problema de la parada).
- El problema es decidible pero no se conocen algoritmos polinomiales que lo resuelvan.

#### Modelos de Computadoras

**Máquina de Turing Determinística** (DTM): Consiste de un control finito, una cabeza lectora-escritora y una cinta con el siguiente esquema.
- Control: corre sobre la cinta
- Cabeza lectora-escritora: lee-escribe en la cinta
- Cinta

Donde:
- $\sum$ finito, el alfabeto; $\Gamma = \Sigma \cup \{\ast\}$;
- $Q$ finito, el conjunto de estados;
- $q_0 \in Q$, estado inicial; $Q_f \subseteq Q$, estados finales ($q_{si}$ y $q_{no}$ para problemas de decisión)

Sobre la cinta tengo escrito el input que es un string de símbolos de $\sum$ a partir de la celda 1, y el resto de las celdas tiene $\ast$ (blancos).  

Definimos un programa $S$ como un conjunto de quíntuplas $S \subseteq Q \times \Gamma \times Q \times \Gamma \times M$, donde $M = \{+1, -1\}$ son los movimientos de la cabeza a derecha o izquierda.  

**Para todo par $(q_i, s_j)$, existe exactamente una quíntupla que comienza con ese par (máquina determinística).**  

¿Qué significa la quíntupla $(q_i, s_h, q_j, s_k, +1)$? Significa que si estando en el estado $q_i$ la cabeza lee $s_h$, entonces escribe $s_k$, se mueve a la derecha y pasa al estado $q_j$.  

Una máquina $M$ resuelve el problema $\pi$ si para toda instancia empieza, termina y contesta bien.  

La complejidad de una DTM está dada por la cantidad de movimientos de la cabeza, desde el estado inicial hasta alcanzar un estado final, en función del tamaño de la entrada.  

### Clase P

Un problema está en P si existe una DTM de complejidad polinomial que lo resuelve.  

- P = { $\pi$ tq $\exists M$ DTM tq $M$ resuelve $\pi$ y $T_M(n) \in O(p(n))$ para algún polinomio $p$ }  

Existen otros modelos de computadoras determinísticas (máquina de Turing con varias cintas, Random Access Machines, etc.) pero puede probarse que son equivalentes en términos de la polinomialidad de los problemas a la DTM.  

**Máquinas de Turing No Determinísticas** (NDTM):

- No se pide unicidad de la quíntupla que comienza con cualquier par $(q_i, s_j)$.  

- En caso de que hubiera más de una quíntupla, la máquina se replica continuando cada una por una rama distinta.

- Decimos que una NDTM resuelve el problema $\pi$ si para toda instancia de $Y_{\pi}$ existe una rama que llega a un estado final $q_{si}$ y para toda instancia en $D_{\pi} \backslash Y_{\pi}$ ninguna rama llega a un estado final $q_{si}$.  

- Una NDTM es polinomial para $\pi$ cuando existe una función polinomial $T(n)$ de manera que para toda instancia de $Y_{\pi}$ de tamaño $n$, alguna de las ramas termina en estado $q_{si}$ en a lo sumo $T(n)$ pasos.  

- Un problema $\pi \in NP$ si existe una NDTM polinomial que resuelve $\pi$.

### Clase NP

Equivalentemente, un problema de decisión pertenece a la clase NP si dada una instancia de SÍ y evidencia de la misma, puede ser verificada en tiempo polinomial. Esa evidencia a veces se llama certificado, y tiene que tener tamaño polinomial en el tamaño de la entrada.  

> **Lema**: Si $\pi$ es un problema de decisión que pertenece a la clase NP, entonces $\pi$ puede ser resuelto por un algoritmo determinístico en **tiempo exponencial** respecto del tamaño de la entrada.  

NP-completitud: Reducción polinomial: Sean $\pi$ y $\pi '$ dos problemas de decisión. Decimos que $f : D_{\pi '} \rightarrow D_{\pi}$ es una reducción polinomial de $\pi '$ en $\pi$ si $f$ se computa en tiempo polinomial y para todo $d \in D_{\pi '}$, $d \in Y_{\pi '} \iff f(d) \in Y_{\pi}$. Notación: $\pi ' \leq \pi$.  

Un problema $\pi$ es NP-completo si:
1. $\pi \in$ NP.
2. Para todo $\pi ' \in$ NP, $\pi ' \leq \pi$.

Si un problema $\pi$ verifica la condición 2., $\pi$ es NP-Hard (es al menos tan “difícil” como todos los problemas de NP).  


**Conjeturas**:
- Si existe un problema en NP-c $\cap$ P, entonces P=NP.
- Si existe un problema en NP \ P, entonces P $\neq$ NP


### Problema de SAT: Teorema de Cook

El problema SAT consiste en decidir si, dada una fórmula lógica $\phi$ expresada como conjunción de disyunciones (ej: $\phi = x_1 \land (x_2 \lor \neg x_1) \land (x_3 \lor \neg x_4 \lor x_1)$ ), existe una valuación de sus variables que haga verdadera $\phi$.

Teorema de Cook (1971): SAT es NP-completo.

La demostración de Cook es directa: considera un problema genérico $\pi \in$ NP y una instancia genérica $d \in D_\pi$. A partir de la hipotética NDTM que resuelve $\pi$, genera en tiempo polinomial una fórmula lógica $\phi_{\pi,d}$ en forma normal (conjunción de disyunciones) tal que $d \in Y_\pi$ si y sólo si $\phi_{\pi,d}$ es satisfactible.  

A partir del Teorema de Cook, la técnica estándar para probar que un problema $π$ es NP-completo aprovecha la transitividad, y consiste en lo siguiente:
1. Mostrar que $π$ está en NP.
2. Elegir un problema $π'$ apropiado que se sepa que es NP-completo.
3. Construir una reducción polinomial $f$ de $π'$ en $π$.


#### Ejemplo: Reducción SAT a 3-SAT

El problema 3-SAT es una variante del problema SAT, en el cual cada cláusula tiene exactamente tres literales. Como es una restricción del dominio de SAT, está en NP, y en principio es "no más difícil" que SAT.  

Para probar que 3-SAT es NP-completo, vamos entonces a reducir SAT a 3-SAT.  

Tomemos una instancia genérica de SAT $ϕ = C_1 ∧ ... ∧ C_m$. Vamos a reemplazar cada $C_i$ por una conjunción de disyunciones $ϕ'_i$, donde cada disyunción tenga tres literales, y de manera que $ϕ$ sea satisfactible si y sólo si $ϕ_1 ∧ ... ∧ ϕ_m$ lo es.  

- Si $C_i$ tiene tres literales, queda como está.
- $C_i$ tiene menos de tres literales, agregamos nuevas variables como en el ejemplo: $(x1 ∨ ¬x2) → (x1 ∨ ¬x2 ∨ y) ∧ (x1 ∨ ¬x2 ∨ ¬y)$
- Si $C_i$ tiene cuatro o más literales, agregamos nuevas variables como en el ejemplo:
$(x1 ∨ ¬x2 ∨ x3 ∨ x4 ∨ ¬x5) → (x1 ∨ ¬x2 ∨ y1) ∧ (¬y1 ∨ x3 ∨ y2) ∧ (¬y2 ∨ x4 ∨ ¬x5)$

### Clase Co-NP

Un problema de decisión pertenece a la clase Co-NP si dada una instancia de NO y evidencia de la misma, puede ser verificada en tiempo polinomial.  

El problema $π^c$ tiene respuesta NO si y solo si $π$ tiene
respuesta SI.  

Co-NP es la clase de los problemas complemento de los problemas de la clase NP. Y la clase de problemas P está contenida también en Co-NP.  

Es una incógnita si Co-NP = NP, o si P = Co-NP $\cup$ NP

#### Extensión de un problema

El problema $π$ es una restricción de un problema $π'$ si el dominio de $π$ está incluido en el de $π'$.  

Se dice que $π'$ es una extensión de $π$. Si $π$ ∈ NP-Completo, entonces $π'$ ∈ NP-Difícil.  

Ejemplos:
- Viajante de comercio es una extensión de Circuito Hamiltoniano
- 3-SAT es una restricción de SAT
