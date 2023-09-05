#include <iostream>
#include "vector"
#include "algorithm"
using namespace std;

// algoritmo greedy
int shopaholic(vector<int> precios){
    int output = 0;
    sort(precios.begin(), precios.end());
    for(int i = precios.size()-1; i >= 2; i -= 3){
        output += precios[i-2];
    }
    return output;
}

int main() {
    int tests;
    cin >> tests;
    int respuestas[tests];
    for(int i = 0; i < tests; i++){
        int cantPrecios;
        cin >> cantPrecios;
        vector<int> precios(cantPrecios, 0);
        for (int j = 0; j < cantPrecios; ++j) {
            int p;
            cin >> p;
            precios[j] = p;
        }
        respuestas[i] = shopaholic(precios);
    }
    for(int r: respuestas){
        if (r == respuestas[tests-1])
            cout << r<< endl;
        else
            cout << r<< endl;
    }
}
