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