# Taller 1: Técnicas Algorítmicas

## Ejercicio 1: _Shopaholic_

Dados 3 productos a comprar, siempre se le aplicará el descuento al más barato de ellos. Entonces mi heurística para el greedy será siempre agarrar los 3 productos más caros posibles y tomar el más barato de ellos para aplicarle el descuento. Supongamos que tenemos los siguientes precios: $[300, 100, 400, 400, 650, 200]$. Entonces el máximo descuento será de $400 + 100$, pues debo tomar primero $[650, 400, 400]$ y su descuento será $400$, y luego me quedan $[300, 200, 100]$ con descuento $100$. Agarrando los 3 productos más caros y aplicarle la promoción sobre ellos maximizará mi descuento total. El algoritmo en `C++` queda:

```cpp
int shopaholic(vector<int> precios){

    int output = 0; // aca sumo el descuento total

    sort(precios.begin(), precios.end()); // lo ordeno para agarrar los máximos facilmente

    for(int i = precios.size()-1; i >= 2; i -= 3){
        output += precios[i-2]; // voy agarrando de a 3 maximos
    }

    return output;
}
```

La complejidad es de $O(n\ log\ n)$ ya que se realiza un sorting y luego se recorre linealmente.


## Ejercicio 2: _SuperSale_

Acá no puedo usar un algoritmo greedy como el del problema del _coin change_ (en el que los valores de las monedas son múltiplos de la más chica de ellas) ya que no siempre tomar el de mayor valor será una solución eficiente. Supongamos que tenemos los objetos $[(100, 70), (50, 35), (55, 35)]$, siendo cada objeto $(Precio, Peso)$, y un único peso disponible de $70$. Entonces agarrar los dos últimos elementos será la solución, en vez de agarrar el primero que es el de mayor precio.  

Tampoco serviría tomar el de mayor coeficiente $Precio / Peso$ porque  

Empiezo con una solución por backtracking de la forma:  

$$
superSale(P, c, j) =
\begin{cases}
     -\infty & \text{si }  \\
     0 & \text{si }  \\
     max & \text{caso contrario}
\end{cases}
$$


## Ejercicio 3: _The Hamming Distance Problem_

