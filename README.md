# Seminario de traductores de lenguajes II

## Analizador léxico
### (completo)

En este módulo se realiza un analizador léxico que diferencia estos símbolos:

| token         | Valor |
|---------------|-------|
| identificador | 0     |
| entero        | 1     |
| real          | 2     |
| cadena        | 3     |
| tipo          | 4     |
| opSuma        | 5     |
| opMul         | 6     |
| opRelac       | 7     |
| opOr          | 8     |
| opAnd         | 9     |
| opNot         | 10    |
| opIgualdad    | 11    |
| ;             | 12    |
| ,             | 13    |
| (             | 14    |
| )             | 15    |
| {             | 16    |
| }             | 17    |
| =             | 18    |
| if            | 19    |
| while         | 20    |
| return        | 21    |
| else          | 22    |
| $             | 23    |

### Pruebas

* Con la cadena "var int float < >= = || 3.5 10 ( } ; ,"
![Texto alternativo](/capturas/11.PNG)