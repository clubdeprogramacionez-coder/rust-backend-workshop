# Sesión 12: dyn Trait y JSONifiable

## 1. Objetivo de Aprendizaje
Entender qué significa usar traits con `dyn` para trabajar con comportamientos polimórficos, y cómo una abstracción como `JSONifiable` permite serializar objetos distintos bajo una misma interfaz.

## 2. Lectura Guiada 
En muchos programas necesitamos trabajar con varios tipos que comparten un comportamiento, pero no todos tienen la misma estructura. Por ejemplo, un sistema puede tener usuarios, productos o pedidos, y todos pueden necesitar convertirse a JSON. En Rust, una forma de modelar esto es con un trait y un puntero dinámico a ese comportamiento.

Cuando se usa `dyn Trait`, se está diciendo: “este valor cumple con este comportamiento, pero el tipo concreto exacto puede variar”. El compilador conoce la interfaz, pero no necesariamente la estructura exacta, y por eso el sistema se vuelve polimórfico sin perder seguridad.

Un ejemplo útil es un trait `JSONifiable`, con un método `to_json()`. Cualquier tipo que implemente ese comportamiento puede ser tratado de manera uniforme. Esta idea es muy parecida a la forma en que un framework trabaja con modelos distintos bajo la misma API.

## 3. Temas de la Sesión
- Traits como interfaces de comportamiento
- `dyn Trait` para polimorfismo
- Abstracción por contratos
- Serialización con un trait compartido

## 4. Código de Explicación
```rust
trait JSONifiable {
    fn to_json(&self) -> String;
}

struct Usuario {
    nombre: String,
}

impl JSONifiable for Usuario {
    fn to_json(&self) -> String {
        format!("{{\"nombre\":\"{}\"}}", self.nombre)
    }
}
```

Aquí el trait `JSONifiable` define una operación común: convertir a JSON. `Usuario` implementa esa capacidad. La lógica de serialización puede ser distinta según el tipo, pero la interfaz de salida es la misma.

```rust
fn imprimir_json(item: &dyn JSONifiable) {
    println!("{}", item.to_json());
}
```

La función acepta cualquier valor que implemente `JSONifiable`, sin importar el tipo concreto que sea. Esto es especialmente útil para crear APIs o sistemas donde se quiere tratar varios tipos con una interfaz unificada.

```rust
let usuario = Usuario { nombre: String::from("Ana") };
imprimir_json(&usuario);
```

Se invoca el comportamiento dinámico sin necesidad de saber internamente cómo está construido el objeto. Esa es una de las grandes ventajas del uso de traits y `dyn` en Rust.

## 5. Tabla Comparativa
| Concepto | Qué representa | Beneficio |
| --- | --- | --- |
| Trait | Contrato de comportamiento | Define una interfaz |
| `dyn Trait` | Referencia a un comportamiento dinámico | Permite polimorfismo |
| Tipo concreto | Implementación real | Tiene datos y lógica específicos |
| `to_json` | Serialización compartida | Se reutiliza para varios tipos |

## 6. LABORATORIO 
*Como:* estudiante que quiere abstraer varias entidades bajo una misma capacidad
*Quiero:* aplicar un trait común a diferentes tipos
*Para:* comprender cómo Rust trata el polimorfismo con `dyn Trait`

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben crear un trait que represente una operación compartida por varios tipos, como serializar a JSON o describir un registro. Luego deben pasar esos tipos a una función que trabaja con la interfaz abstracta en lugar del tipo concreto.

Deberán:
- Definir un trait con un método común.
- Implementarlo para dos tipos distintos.
- Usar `&dyn Trait` en una función.
- Explicar la diferencia entre trabajar con tipos concretos y con un comportamiento compartido.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Definí un trait con comportamiento común.
- [ ] Implementé el trait en distintos tipos.
- [ ] Usé `dyn Trait` para abstraer el comportamiento.
- [ ] Relacioné esto con el polimorfismo y la serialización.

## 9. Pregunta de Cierre
Si varios tipos necesitan compartir una acción como convertir a JSON, ¿por qué un trait con `dyn` ofrece una abstracción útil sin perder la seguridad que Rust exige?

