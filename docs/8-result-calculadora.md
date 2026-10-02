# Sesión 8: Result y calculadora

## 1. Objetivo de Aprendizaje
Entender cómo Rust modela errores con `Result`, y usar esta idea para construir una calculadora que maneje casos inválidos como la división entre cero o una operación no soportada.

## 2. Lectura Guiada 
No todos los fallos son ausencia de valor. A veces un cálculo puede fallar por una condición específica: dividir entre cero, una entrada inválida o una operación imposible. Rust no usa excepciones como en otros lenguajes; usa el tipo `Result<T, E>`.

`Result` tiene dos variantes:
- `Ok(T)`: la operación tuvo éxito y devuelve un valor.
- `Err(E)`: la operación falló y devuelve información del error.

Esto hace que el código muestre de forma clara dónde puede haber una falla. Si una función puede fracasar, el compilador obliga a considerarla. No se puede ignorar un error sin manejarlo intencionalmente.

Cuando construimos una calculadora, esto es especialmente útil porque cada operación puede fallar por distintos motivos. La lógica del programa queda más expresiva y segura, porque el error se vuelve parte del contrato de la función.

## 3. Temas de la Sesión
- `Result<T, E>` para errores controlados
- `Ok` y `Err`
- Manejo de errores con `match`
- Diseño de una calculadora robusta

## 4. Código de Explicación
```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("No se puede dividir entre cero"))
    } else {
        Ok(a / b)
    }
}
```

Esta función representa una operación que puede fallar. Si el divisor es cero, devuelve `Err(...)`; si no, devuelve `Ok(...)` con el resultado. Así, el llamador sabe exactamente qué esperar.

```rust
match dividir(10.0, 2.0) {
    Ok(resultado) => println!("Resultado: {}", resultado),
    Err(error) => println!("Error: {}", error),
}
```

La estructura `match` hace explícito el manejo del éxito y el fracaso. La respuesta no es ambigua: el programa decide qué imprimir en cada caso.

```rust
enum Operacion {
    Suma(f64, f64),
    Resta(f64, f64),
    Multiplicacion(f64, f64),
    Division(f64, f64),
}
```

Con un enum como este se modela una calculadora con varias operaciones, y cada variante puede llevar los valores asociados. Luego la lógica de cálculo puede devolver `Result` para representar cualquier error de entrada o de operación.

## 5. Tabla Comparativa
| Concepto | `Result<T, E>` | `Option<T>` |
| --- | --- | --- |
| Casos | `Ok` o `Err` | `Some` o `None` |
| Significado | Operación exitosa o fallida | Valor presente o ausente |
| Error | Se incluye información del error | No existe un error, solo ausencia |
| Uso típico | Validación, cálculos, I/O | Datos opcionales |

## 6. LABORATORIO 
*Como:* estudiante que quiere construir lógica con validación
*Quiero:* diseñar una calculadora que informe errores claros
*Para:* practicar control de flujo y manejo de fallas en Rust

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben crear una pequeña calculadora con operaciones básicas. Cada operación debe devolver `Result`, para que el programa valide entradas y responda con un mensaje de error cuando algo no es posible.

Deberán:
- Definir al menos dos o tres operaciones aritméticas.
- Validar casos de error, como división entre cero.
- Devolver `Result` en cada operación.
- Mostrar al usuario una respuesta clara según el caso.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Definí operaciones para la calculadora.
- [ ] Manejé errores con `Result`.
- [ ] Validé casos inválidos como división entre cero.
- [ ] Explicité cómo el programa responde a fallos previstos.

## 9. Pregunta de Cierre
Si una operación puede tener éxito o fallar por una razón clara, ¿por qué `Result` ayuda más que simplemente usar un valor arbitrario o un `println!` sin estructura de control?

