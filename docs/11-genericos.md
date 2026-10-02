# Sesión 11: Genéricos

## 1. Objetivo de Aprendizaje
Aprender a escribir funciones y tipos genéricos para reutilizar código sin perder seguridad de tipos y sin duplicar lógica para cada estructura concreta.

## 2. Lectura Guiada 
A veces queremos escribir una función que funcione con enteros, cadenas, números con punto flotante o incluso tipos personalizados, sin repetir el mismo algoritmo. En Rust, esto se logra con genéricos.

Los genéricos permiten definir una función o un tipo con parámetros de tipo, como `T` o `U`. La sintaxis es simple: `fn mayor<T: PartialOrd>(a: T, b: T) -> T`. Eso indica que la función acepta cualquier tipo `T` que se pueda comparar con orden parcial.

Esto tiene dos grandes ventajas: reduce duplicación de código y mantiene la seguridad del compilador. Rust verifica que el tipo usado es compatible con las operaciones que la función exige. De esta forma, escribimos código más flexible sin perder rigor.

## 3. Temas de la Sesión
- Funciones genéricas
- Tipos genéricos
- Restricciones con traits
- Reutilización y seguridad

## 4. Código de Explicación
```rust
fn mayor<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
```

La función `mayor` puede trabajar con cualquier tipo `T` que pueda compararse. Eso incluye tipos numéricos o incluso tipos propios si implementan la comparación adecuada. La restricción `PartialOrd` expresa esa capacidad.

```rust
let x = mayor(10, 20);
let y = mayor(3.5, 8.1);
println!("{} {}", x, y);
```

Aquí vemos el valor de los genéricos. La misma lógica sirve para varios tipos, siempre que compartan la capacidad de compararse. Esto evita reescribir la misma función para cada tipo numérico.

```rust
struct Contenedor<T> {
    valor: T,
}

let c = Contenedor { valor: String::from("Rust") };
```

Los genéricos no solo se usan en funciones; también en structs. Un contenedor puede guardar cualquier tipo de dato, pero el tipo se decide en el momento de instanciarlo. Esto los hace muy útiles para colecciones y abstracciones.

## 5. Tabla Comparativa
| Concepto | Descripción | Ejemplo |
| --- | --- | --- |
| Función genérica | Reutiliza lógica para varios tipos | `fn mayor<T>(...)` |
| Tipo genérico | Almacena cualquier tipo | `struct Contenedor<T>` |
| Restricción | Exige que el tipo implemente un trait | `T: PartialOrd` |
| Beneficio | Código flexible y seguro | Reutilización sin perder validación |

## 6. LABORATORIO 
*Como:* estudiante que quiere escribir código más reutilizable
*Quiero:* crear funciones y estructuras genéricas
*Para:* entender cómo Rust combina flexibilidad con seguridad de tipos

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben crear una función o una estructura genérica que pueda aceptar distintos tipos, siempre que cumplan una capacidad específica. Deben explicar la razón de las restricciones y el valor de reutilizar el mismo algoritmo.

Deberán:
- Definir al menos una función genérica.
- Agregar una restricción con trait.
- Usar la función con varios tipos compatibles.
- Explicar la ventaja de evitar duplicación de código.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Usé genéricos en una función o struct.
- [ ] Agregué restricciones con traits.
- [ ] Validé que funciona con varios tipos.
- [ ] Explicité por qué esto mejora reutilización y seguridad.

## 9. Pregunta de Cierre
Si un mismo algoritmo puede aplicarse a varios tipos, ¿por qué es mejor usar genéricos en Rust que repetir la lógica para cada tipo manualmente?

