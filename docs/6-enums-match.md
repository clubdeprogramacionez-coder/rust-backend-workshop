# Sesión 6: Enums y Match

## 1. Objetivo de Aprendizaje
Comprender cómo Rust modela estados y decisiones con enums y pattern matching, usando `match` como una forma clara, segura y exhaustiva de evaluar cada caso posible.

## 2. Lectura Guiada 
Pattern matching es una de las ideas más importantes de Rust porque permite que el programa responda a datos según su forma o estado, sin mezclar condiciones difíciles de seguir. Imagina un semáforo: cada color tiene un significado distinto y el sistema debe reaccionar según el estado actual. Un enum funciona igual: define un conjunto de estados posibles para una entidad. Luego, `match` compara ese valor con cada caso y ejecuta la lógica correcta.

Por ejemplo, una cuenta puede estar en estado `Activo`, `Inactivo` o `Suspendido`. En lugar de manejar varias variables booleanas o condiciones encadenadas, Rust permite describir esos estados como variantes de un tipo. Después, `match` revisa cada una y exige que se cubran todos los casos posibles. Eso es lo que se llama exhaustividad: el compilador verifica que no se olvide ningún estado.

Esta forma de programación hace que el código sea más legible y más segura. En lugar de “si pasa esto, si pasa aquello”, el programa expresa los posibles escenarios de forma directa. Además, cuando el enum lleva datos asociados, `match` puede extraer esos datos automáticamente y usarlos en cada caso. Es una manera ordenada de pensar en estados, decisiones y flujo del programa.

## 3. Temas de la Sesión
- Enum como representación de estados
- Variantes con valores asociados
- Pattern matching con `match`
- Exhaustividad y claridad en decisiones

## 4. Código de Explicación
```rust
enum Estado {
    Activo,
    Inactivo,
    Suspendido,
}
```

Este enum define tres estados posibles. `Activo`, `Inactivo` y `Suspendido` son variantes, y cada una representa una situación distinta del mismo concepto: el estado de un sistema o de una entidad. En vez de usar números o cadenas sueltas, Rust hace explícito qué estados pueden existir.

```rust
fn describir(estado: Estado) -> &'static str {
    match estado {
        Estado::Activo => "El sistema está disponible",
        Estado::Inactivo => "El sistema está detenido",
        Estado::Suspendido => "El sistema requiere revisión",
    }
}
```

Aquí aparece el patrón principal: `match`. La expresión `estado` se compara con cada variante del enum. Si es `Estado::Activo`, se devuelve el texto asociado. Si es `Estado::Inactivo`, devuelve otra respuesta. Si es `Estado::Suspendido`, otra distinta. La clave es que Rust exige que todas las variantes se manejen. No se puede olvidar un caso porque el compilador lo detecta.

`match` no es solo una estructura de control; es una forma de verificar patrones y descomponer valores. Cuando una variante lleva datos, también se pueden extraer esos valores dentro del patrón, por ejemplo `Estado::Mensaje(texto)` o `Estado::Error(codigo)`. Eso hace que la comparación no solo decida qué camino tomar, sino que también extraiga información útil para el caso correcto.

## 5. Tabla Comparativa
| Concepto | Qué representa | Uso principal |
| --- | --- | --- |
| `enum` | Un conjunto de estados o opciones | Modelar dominio |
| Variante | Cada posibilidad del enum | Diferenciar casos |
| `match` | Comparación exhaustiva | Decidir por caso |
| `if` | Condición booleana | Casos simples |

## 6. LABORATORIO 
*Como:* estudiante que ya sabe modelar datos y estados
*Quiero:* representar un conjunto finito de posibilidades con enum y evaluarlas con match
*Para:* comprender cómo Rust organiza decisiones y evita olvidar casos del sistema

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben crear un enum que represente varios estados de un sistema o entidad. Luego deben usar `match` para describir qué ocurre en cada caso y comprobar que todos los estados están cubiertos.

Deberán:
- Definir al menos tres variantes del enum.
- Asociar una lógica clara a cada caso.
- Usar `match` para evaluar el valor recibido.
- Explicar por qué se requiere cubrir todas las opciones posibles.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Definí al menos tres estados.
- [ ] Usé un enum para modelarlos.
- [ ] Manejé cada caso con `match`.
- [ ] Explicité por qué la exhaustividad importa.

## 9. Pregunta de Cierre
Si un sistema puede estar en varios estados y cada uno requiere una respuesta distinta, ¿por qué `match` resulta más claro y seguro que hacer varias condiciones `if` encadenadas?