

## Ejercicio 16

Flojo de demostración este, pero en YouTube lo explica uno [en este video 🔍](https://youtu.be/YBDJ2jZCCQs?si=dnYutkb17IJkYn39&t=280)

```rust
pub fn gasolineras(p: & Vec<i32>, mut c: i32) -> Vec<i32>{
    let mut sol: Vec<i32> = vec![];
    let c_og = c;
    for i in 0..p.len()-1{
        if p[i+1]-p[i] <= c {
            c = c - (p[i+1]-p[i]); // todavia alcanza asi que gasto nafta hasta sgte parada
        }
        else{
            sol.push(p[i]);
            c = c_og - (p[i+1]-p[i]); // cargo nafta y gasto hasta la sgte parada
        }
    }
    return sol;
}
```

