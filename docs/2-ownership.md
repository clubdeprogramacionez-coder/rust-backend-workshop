# Sesión 2: Ownership

## 1. Objetivo de Aprendizaje
Entender el concepto de ownership en Rust, que define quién es el dueño de un valor y cómo el lenguaje evita errores comunes de memoria y uso de datos.

## 2. Lectura Guiada 
Ownership es una regla de sentido común convertida en sistema. Piensa en una casa: una persona puede ser dueña de la llave del inmueble; si esa llave se entrega a otra persona, la primera ya no puede usarla como si fuera suya. Rust aplica algo parecido con los valores en memoria. Cada valor tiene un único dueño y ese dueño es la variable que lo creó. Cuando la variable deja de usarse, Rust libera la memoria automáticamente.

La regla principal es sencilla: un valor solo puede tener un dueño a la vez. Si lo movemos a otra variable, la variable anterior ya no puede acceder a ese dato. Esto parece estricto, pero es exactamente lo que evita bugs como usar un valor después de que ya fue liberado o modificarlo desde dos referencias a la vez sin control.

Esto también ayuda a entender el ciclo de vida de los datos. El valor nace cuando se crea, vive mientras la variable que lo posee siga en scope y desaparece cuando sale de ese scope. Si una función recibe un valor, la propiedad puede transferirse. Si queremos reutilizarlo, debemos decidir si lo clonamos, lo devolvemos o lo prestamos temporalmente. En otras palabras, ownership no es un detalle opcional: es la base de la seguridad de Rust.

## 3. Temas de la Sesión
- Dueño único de cada valor
- Movimiento de datos entre variables
- Scope y liberación automática
- Prevención de errores por uso inválido

## 4. Código de Explicación
```rust
let nombre = String::from("Ana");
let copia = nombre;
println!("{}", copia);
```

La línea `let nombre = String::from("Ana")` crea un valor en memoria con dueño inicial: `nombre`. Luego aparece `let copia = nombre;`. Aquí se mueve la propiedad del valor a `copia`. Eso significa que `nombre` ya no es el dueño del dato. La siguiente línea `println!("{}", copia)` sí es válida porque `copia` es la variable que ahora posee el valor. Si intentáramos usar `nombre` después del movimiento, Rust no lo permitiría y el compilador nos señalaría el error.

```rust
fn saluda() {
    let texto = String::from("hola");
    println!("{}", texto);
}
```

Aquí el valor `texto` se crea dentro de la función. Cuando la función termina, `texto` sale de scope y Rust libera la memoria. Esto es importante porque la reasignación y la destrucción de recursos no dependen de nosotros manualmente; el compilador y el runtime gestionan eso de acuerdo con la regla de ownership. La idea es que el sistema tenga menos margen para errores humanos.

## 5. Tabla Comparativa
| Concepto | Ownership en Rust | En muchos lenguajes |
| --- | --- | --- |
| Dueño | Solo una variable lo posee | Puede haber varias referencias |
| Transferencia | Se mueve la propiedad | Se copia o se comparte sin control |
| Liberación | Automática al salir de scope | Requiere gestión manual o GC |
| Error común | Uso después de mover | Punteros colgantes o abuso de memoria |

## 6. LABORATORIO 
*Como:* estudiante que está aprendiendo la regla de propiedad
*Quiero:* reconocer cuándo una variable mueve un valor y cuándo el valor se libera
*Para:* visualizar la lógica que protege a Rust de errores comunes de memoria

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben analizar varios ejemplos donde una variable crea un valor y luego se asigna a otra. Deben identificar qué variable es dueña del dato, cuándo se produce un movimiento y qué ocurre cuando el valor sale de scope.

Deberán:
- Comparar ejemplos de variables simples y cadenas.
- Señalar cuándo hay movimiento de propiedad.
- Explicar qué sucede al final de una función.
- Describir por qué Rust evita el uso de un valor después de ser movido.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Reconocí qué variable es dueña del valor.
- [ ] Explicité qué significa mover un valor.
- [ ] Diferencié scope, propiedad y transferencia.
- [ ] Relacioné ownership con la seguridad del código.

## 9. Pregunta de Cierre
Si una variable pasa la propiedad a otra, ¿qué pasa con la primera y por qué Rust decide impedir el uso posterior de la variable original?