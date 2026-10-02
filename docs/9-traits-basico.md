# Sesión 9: Traits básicos

## 1. Objetivo de Aprendizaje
Comprender qué son los traits en Rust, cómo sirven para definir comportamientos compartidos y cómo se implementan para tipos distintos sin duplicar lógica.

## 2. Lectura Guiada 
En Rust, un trait es como un contrato: dice “si este tipo quiere comportarse así, debe implementar este conjunto de métodos”. Es una manera de describir capacidades comunes, como “se puede imprimir”, “se puede comparar” o “se puede dibujar”.

Pensemos en un ejemplo de un videojuego: varios objetos pueden ser dibujables: un personaje, un enemigo, un fondo o un proyectil. Todos comparten la idea de tener un método `dibujar()`, pero cada tipo lo implementa de manera distinta. El trait define la interfaz; cada estructura define la lógica particular.

Los traits permiten escribir funciones que acepten cualquier tipo que cumpla cierta capacidad. Así, el código se vuelve más reutilizable y más seguro, porque el compilador verifica en tiempo de compilación que los tipos tienen las operaciones requeridas.

## 3. Temas de la Sesión
- Definición de traits
- Implementación de métodos para tipos
- Reutilización de comportamiento
- Contratos de tipo en Rust

## 4. Código de Explicación
```rust
trait Saludar {
    fn saludar(&self);
}

struct Persona {
    nombre: String,
}

impl Saludar for Persona {
    fn saludar(&self) {
        println!("Hola, soy {}", self.nombre);
    }
}
```

Aquí se define el trait `Saludar` con un método llamado `saludar`. Después se implementa para `Persona`. Esto significa que cualquier `Persona` puede usar ese comportamiento de forma natural.

```rust
fn mostrar_saludo<T: Saludar>(valor: &T) {
    valor.saludar();
}
```

La función genérica `mostrar_saludo` acepta cualquier tipo que implemente `Saludar`. Es decir, el compilador exige que el tipo cumpla con el contrato antes de permitir su uso. Esto es una base clave de la programación con traits.

```rust
let p = Persona { nombre: String::from("Ana") };
mostrar_saludo(&p);
```

El código se ejecuta porque `Persona` implementa `Saludar`. La función puede manejar cualquier tipo compatible sin conocer detalles específicos de cada estructura.

## 5. Tabla Comparativa
| Elemento | Qué representa | Beneficio |
| --- | --- | --- |
| Trait | Contrato o comportamiento | Define capacidades |
| `impl Trait for Type` | Implementación | Asocia comportamiento con un tipo |
| Función genérica | Reutilización | Acepta varios tipos compatibles |
| Tipo concreto | Datos reales | Tiene comportamiento definido |

## 6. LABORATORIO 
*Como:* estudiante que quiere abstraer comportamiento común
*Quiero:* definir un trait y usarlo en varios tipos
*Para:* entender cómo Rust reutiliza lógica sin duplicar código

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben crear un trait con un comportamiento simple, implementarlo para dos tipos distintos y mostrar cómo ambos comparten la misma capacidad aunque tengan datos distintos.

Deberán:
- Definir un trait con al menos un método.
- Crear dos structs diferentes.
- Implementar el trait en ambos.
- Usar una función genérica que acepte cualquier tipo compatible.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Definí un trait con un comportamiento útil.
- [ ] Implementé el trait en varios tipos.
- [ ] Validé que el código reutiliza lógica.
- [ ] Explicité la diferencia entre contrato y tipo concreto.

## 9. Pregunta de Cierre
Si varios tipos comparten una capacidad, ¿por qué los traits son una forma más elegante y segura de reutilizar comportamiento que copiar la misma función para cada estructura?

