# Sesión 5: Structs e Implementaciones

## 1. Objetivo de Aprendizaje
Entender cómo Rust modela información real con structs y cómo los métodos definidos en impl permiten asociar comportamiento a esos datos de una forma clara y segura.

## 2. Lectura Guiada 
Cuando trabajamos con programas reales, rara vez queremos guardar solo un número o una cadena aislada. Necesitamos representar entidades completas: una persona, un producto, una cuenta, un mensaje. Un struct sirve para agrupar varios datos relacionados bajo una sola entidad. Por ejemplo, una persona puede tener nombre, edad y ciudad; un producto puede tener nombre, precio y stock.

La clave de un struct no es solo guardar información, sino organizarla con sentido. En vez de manejar variables sueltas, un struct crea una “forma” más natural para representar algo del mundo real. Además, con `impl` se pueden definir métodos, es decir, funciones que actúan sobre esos datos. Eso permite que el código sea más legible y más cercano a la lógica del problema.

Esto también ayuda a mantener el programa ordenado. En lugar de dispersar datos en muchas variables a lo largo del código, se agrupan bajo una estructura con nombre. Rust usa esta organización como parte de un diseño más claro, y la seguridad del lenguaje sigue siendo importante porque los datos y sus operaciones trabajan juntos de manera controlada.

## 3. Temas de la Sesión
- Definición de structs para agrupar datos
- Campos y estados de una entidad
- Métodos con `impl`
- Organización de código y comportamiento asociado

## 4. Código de Explicación
```rust
struct Persona {
    nombre: String,
    edad: u8,
}
```

El `struct Persona` define una entidad con dos campos: `nombre` y `edad`. Cada campo representa una parte de la información. En vez de manejar datos separados, se agrupan porque pertenecen a la misma idea: una persona. Esto hace que el código sea más claro y que la estructura de datos refleje mejor el problema que se está resolviendo.

```rust
impl Persona {
    fn saludar(&self) {
        println!("Hola, soy {} y tengo {} años.", self.nombre, self.edad);
    }
}
```

La palabra `impl` define una implementación para `Persona`. El método `saludar` recibe `&self`, es decir, una referencia inmutable al propio objeto. Con esto, el método puede leer los datos sin tomar posesión del valor. La línea `println!` muestra el nombre y la edad. El resultado es una operación asociada a la entidad, que hace que el código tenga un sentido más expresivo y natural.

## 5. Tabla Comparativa
| Elemento | Qué representa | Beneficio |
| --- | --- | --- |
| Struct | Entidad con varios datos | Agrupa información relacionada |
| Campo | Atributo del dato | Describe el estado |
| Método | Comportamiento | Define acciones sobre el dato |
| `impl` | Bloque de lógica | Organiza funciones relacionadas |

## 6. LABORATORIO 
*Como:* estudiante que quiere modelar información con sentido
*Quiero:* definir una estructura de datos y asociarle comportamiento
*Para:* comprender cómo Rust organiza entidades reales en un programa más claro y mantenible

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben definir un struct con datos reales de un caso sencillo, por ejemplo una persona, un producto, un estudiante o una cuenta. Luego deben asociar uno o dos métodos para representar comportamientos básicos de esa entidad.

Deberán:
- Crear una estructura con varios campos.
- Asignar nombres claros a cada propiedad.
- Definir al menos un método usando `impl`.
- Explicar cómo los datos y las acciones quedan unidos en una sola entidad.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Modelé una entidad útil.
- [ ] Definí sus campos con sentido.
- [ ] Agregué una operación asociada.
- [ ] Explicité la ventaja de organizar datos en un solo struct.

## 9. Pregunta de Cierre
Si una entidad real del mundo puede describirse con varios atributos y comportamientos, ¿por qué es útil representarla como un struct en Rust en lugar de manejar variables independientes?