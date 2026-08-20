# Sesión 1: Stack y Heap

## 1. Objetivo de Aprendizaje
Comprender cómo Rust organiza la memoria en stack y heap, y por qué esa decisión influye en el rendimiento, el tamaño de los datos y la seguridad del programa.

## 2. Lectura Guiada 
Imagina una oficina con dos lugares para guardar materiales. El escritorio del trabajador es el stack: todo está ordenado, es pequeño, rápido y se usa en el mismo orden en que se necesita. Cuando una variable entra en scope, el programa coloca su valor allí si sabe exactamente cuántos bytes ocupa. En cambio, el heap es como un almacén grande donde se guarda información cuyo tamaño no es fijo o que necesita vivir más tiempo que una sola función.

En Rust, esta diferencia importa mucho porque la memoria no es solo una caja donde guardar cosas: también define cómo vive y desaparece cada dato. Si el valor es pequeño y conocido, normalmente se guarda en stack. Si es más complejo, como una cadena que puede crecer o cambiar, se guarda en heap y se accede mediante una referencia. Esto explica por qué Rust puede ser muy seguro: el lenguaje separa claramente qué datos tienen un tamaño fijo y qué datos necesitan almacenamiento dinámico.

El stack funciona como una pila de platos: se apilan y se quitan en el último orden en que se usaron. El heap es más parecido a un depósito donde puedes guardar cosas sin saber de antemano cuántas caben, pero con un costo extra de organización y acceso. Por eso, si un dato es pequeño y temporal, stack es ideal; si es grande o cambiante, heap es la opción.

## 3. Temas de la Sesión
- Stack: memoria rápida y de tamaño conocido
- Heap: memoria dinámica y de mayor flexibilidad
- Variables locales y tiempo de vida corto
- Relación entre memoria y seguridad en Rust

## 4. Código de Explicación
```rust
let entero = 42;
let texto = String::from("Rust");
println!("{} {}", entero, texto);
```

Primero se declara `entero` con valor `42`. Ese número es pequeño y su tamaño es fijo, por lo que puede vivir en stack sin problema. La variable `texto` no es un número simple: es una cadena que puede crecer y cambiar, así que Rust la representa con datos que necesitan más espacio y un manejo especial. Esa información se reserva en heap, mientras que la variable misma sigue teniendo una referencia o estructura en stack.

La línea `println!` muestra que el programa puede leer ambos valores y mostrarlos en pantalla. Lo importante es entender que el valor `entero` se almacena directamente y el valor `texto` se administra con más lógica, porque su contenido no es fijo en tamaño.

```rust
fn ejemplo() {
    let a = 10;
    let b = String::from("curso");
    println!("a = {}", a);
    println!("b = {}", b);
}
```

`a` es un entero de tamaño fijo, por lo tanto se guarda en stack. `b` es una cadena, así que Rust reserva espacio para sus datos en heap y maneja la estructura de la cadena en stack. Cuando la función termina, `a` y `b` salen de scope y su memoria se libera. La clave es que Rust decide esto de forma automática y segura, evitando que el programa acceda a memoria que ya no debería usar.

## 5. Tabla Comparativa
| Aspecto | Stack | Heap |
| --- | --- | --- |
| Tamaño | Conocido en tiempo de compilación | Dinámico y variable |
| Velocidad | Muy rápida | Más lenta |
| Organización | LIFO | Más flexible |
| Uso típico | Enteros, referencias pequeñas, variables locales | Cadenas, listas, estructuras grandes |
| Vida | Corto y local | Puede durar más tiempo |

## 6. LABORATORIO 
*Como:* estudiante de programación inicial
*Quiero:* identificar qué datos de un programa permanecen en stack y cuáles se reservan en heap
*Para:* comprender la base de la memoria en Rust y preparar el camino hacia ownership y referencias

## 7. PRÁCTICA - Instrucciones para Classroom
En este ejercicio deben analizar un ejemplo simple de Rust y decidir qué variables se almacenan en stack y qué datos se requieren en heap. Deben explicar visualmente la diferencia entre un valor fijo y un valor dinámico, y comparar cómo cambia el comportamiento cuando una variable se vuelve más compleja.

Deberán:
- Revisar el archivo de laboratorio correspondiente y reconocer cada variable por tipo.
- Identificar qué datos tienen tamaño fijo y cuáles no.
- Describir cómo se comporta cada valor al entrar y salir de una función.
- Comentar en texto qué parte del programa se maneja en stack y qué parte requiere heap.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Identifiqué qué valores se guardan en stack.
- [ ] Identifiqué qué valores necesitan heap.
- [ ] Explicité por qué algunos datos tienen tamaño fijo y otros no.
- [ ] Relacioné la memoria con la seguridad del lenguaje.

## 9. Pregunta de Cierre
Si un programa guarda una cadena de texto y un número entero, ¿cuál de los dos necesita memoria más dinámica y por qué crees que Rust trata cada caso de manera diferente?