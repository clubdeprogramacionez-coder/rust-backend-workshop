# Sesión 7: Option y manejo de ausencia

## 1. Objetivo de Aprendizaje
Comprender cómo Rust representa la ausencia de valor con `Option`, evitando el problema de `null` y forzando a manejar explícitamente los casos válidos e inválidos.

## 2. Lectura Guiada 
En muchos lenguajes, una variable puede ser `null` o `undefined`, y el programador debe recordar comprobarlo antes de usarla. Rust evita ese problema con un tipo especial llamado `Option`. En lugar de permitir que un dato “pueda no existir”, el lenguaje lo expresa de forma explícita en el tipo.

`Option<T>` tiene dos variantes: `Some(T)` cuando el valor existe y `None` cuando no existe. Esto hace que el código sea más seguro porque el compilador exige manejar los dos escenarios. Una función que podría no devolver un valor no devuelve un `T` cualquiera, sino un `Option<T>`.

Esto cambia mucho la manera de pensar: no se “adivina” si algo pudo ser nulo, sino que se declara en el tipo. Al mismo tiempo, `match` se convierte en una herramienta natural para decidir qué hacer en cada caso. Es una forma clara de transformar la posibilidad de ausencia en una decisión consciente del programador.

## 3. Temas de la Sesión
- `Option<T>` como representación de valor opcional
- `Some` y `None`
- Manejo explícito con `match`
- Prevención de errores por valores faltantes

## 4. Código de Explicación
```rust
fn buscar_nombre() -> Option<String> {
    let nombre = String::from("Ana");
    Some(nombre)
}
```

La función `buscar_nombre` devuelve `Option<String>`. Eso significa que no siempre habrá un texto de retorno, pero si existe, será un `String` dentro de `Some(...)`. La ausencia se expresa como `None`, y el valor real se encapsula en `Some(...)`.

```rust
let valor: Option<i32> = Some(10);

match valor {
    Some(numero) => println!("El valor es {}", numero),
    None => println!("No hay valor"),
}
```

Aquí se ve la clave de `Option`: el valor debe manejarse con `match`. Si es `Some`, se obtiene el entero y puede usarse directamente; si es `None`, el programa responde de forma segura y consciente, sin riesgo de acceder a un dato inexistente.

```rust
fn obtener_edad(edad: Option<u8>) -> u8 {
    match edad {
        Some(e) => e,
        None => 0,
    }
}
```

Este patrón es muy útil porque convierte una posible ausencia en una respuesta explícita. El código no “asume” que hay un valor; obliga a decidir qué hacer cuando no lo hay.

## 5. Tabla Comparativa
| Concepto | `Option<T>` | Valor directo `T` |
| --- | --- | --- |
| Significado | Puede existir o no | Siempre existe |
| Variante con valor | `Some(T)` | `T` |
| Variante sin valor | `None` | No aplica |
| Seguridad | Obligatorio manejar ambos casos | Puede producir errores lógicos |
| Uso típico | Datos opcionales o faltantes | Datos siempre presentes |

## 6. LABORATORIO 
*Como:* estudiante que quiere manejar datos que pueden ser inexistentes
*Quiero:* usar `Option` para modelar ausencia de valor
*Para:* evitar errores por `null` y escribir programas más seguros

## 7. PRÁCTICA - Instrucciones para Classroom
En este ejercicio deben crear una función que reciba una entrada opcional y decida qué hacer cuando el valor existe o no. La idea es practicar la diferencia entre manejar un dato real y una ausencia explícita.

Deberán:
- Definir un dato que pueda ser opcional.
- Usar `Option<T>` para representarlo.
- Evaluar todos los casos con `match`.
- Explicar por qué Rust evita el uso de valores no existentes.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Representé una ausencia con `Option`.
- [ ] Manejé `Some` y `None` explícitamente.
- [ ] Usé `match` para decidir el flujo del programa.
- [ ] Relacioné esto con la seguridad del lenguaje.

## 9. Pregunta de Cierre
Si una operación puede devolver un valor o no devolver ninguno, ¿por qué `Option` resulta más seguro y claro que simplemente permitir `null` o un valor especial arbitrario?

