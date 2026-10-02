# Sesión 10: Traits estándar

## 1. Objetivo de Aprendizaje
Conocer algunos traits estándar de Rust, como `Display`, `Debug`, `Clone` y `PartialEq`, y entender cómo ayudan a imprimir, comparar y manipular valores de manera idiomática.

## 2. Lectura Guiada 
Rust incluye un conjunto de traits que forman parte de la experiencia del lenguaje. No son solo una librería adicional: son patrones ampliamente utilizados que permiten que tipos comunes se comporten de forma esperada en distintas situaciones.

`Debug` permite formatear un valor para depuración con `{:?}`. `Display` permite mostrarlo como texto legible con `{}`. `Clone` permite crear una copia explícita del valor, y `PartialEq` permite comparar dos valores con `==`.

Esto tiene un objetivo muy importante: mantener un estilo consistente y seguro. El compilador ayuda a saber cuándo una operación es válida y cuándo no, y el código se vuelve más claro porque la intención del programador queda reflejada en los traits implementados por los tipos.

## 3. Temas de la Sesión
- `Debug` para depuración
- `Display` para texto legible
- `Clone` para copiar valores
- `PartialEq` para comparación

## 4. Código de Explicación
```rust
#[derive(Debug, Clone, PartialEq)]
struct Persona {
    nombre: String,
    edad: u8,
}
```

El atributo `derive` genera automáticamente la implementación de varios traits para `Persona`. Así, el tipo puede imprimirse con `{:?}`, clonarse y compararse con `==` sin escribir manualmente toda esa lógica.

```rust
let p1 = Persona {
    nombre: String::from("Ana"),
    edad: 21,
};

println!("{:?}", p1);
let p2 = p1.clone();
println!("{}", p1 == p2);
```

Esto es muy práctico en programas reales. Cuando se depura una estructura, se puede imprimir el valor para inspeccionarlo. Cuando se quiere hacer una copia, `Clone` lo permite de forma explícita. Y cuando se quiere comparar dos instancias, `PartialEq` hace esa operación natural.

```rust
impl std::fmt::Display for Persona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.nombre, self.edad)
    }
}
```

`Display` permite que un tipo tenga una representación textual amigable. Esto mejora mucho la salida del programa y hace que los mensajes al usuario sean claros.

## 5. Tabla Comparativa
| Trait | Uso principal | Ejemplo de salida |
| --- | --- | --- |
| `Debug` | Depurar y inspeccionar | `Persona { nombre: "Ana", edad: 21 }` |
| `Display` | Mostrar texto amigable | `Ana (21)` |
| `Clone` | Copiar valor | `p2 = p1.clone()` |
| `PartialEq` | Comparar miembros | `p1 == p2` |

## 6. LABORATORIO 
*Como:* estudiante que quiere trabajar con tipos de datos reales
*Quiero:* imprimir, comparar y copiar estructuras con traits estándar
*Para:* aprender la convención idiomática de Rust

## 7. PRÁCTICA - Instrucciones para Classroom
En este ejercicio deben crear un struct con algunos campos y activar traits estándar para depurar, comparar y mostrar el valor. Después deben explicar cómo esos traits facilitan el trabajo diario con datos complejos.

Deberán:
- Definir un struct con varios campos.
- Usar `derive` para `Debug`, `Clone` y `PartialEq`.
- Implementar `Display` para una salida legible.
- Mostrar el resultado de comparar e imprimir el valor.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Usé traits estándar del lenguaje.
- [ ] Implementé `Debug` y/o `Display`.
- [ ] Validé copia y comparación con `Clone` y `PartialEq`.
- [ ] Relacioné esta práctica con la ergonomía y seguridad de Rust.

## 9. Pregunta de Cierre
¿Por qué es útil que Rust tenga traits estándar para depurar, mostrar y comparar tipos, en lugar de depender de lógica manual para cada estructura?

