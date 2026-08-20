# Sesión 3: Borrowing y Referencias

## 1. Objetivo de Aprendizaje
Aprender a prestar acceso a un valor sin transferir la propiedad, entendiendo la diferencia entre referencias inmutables, referencias mutables y el principio de exclusividad en Rust.

## 2. Lectura Guiada 
Ownership dice que una variable es dueña de un valor. Pero no siempre necesitamos ceder la propiedad completa para leer o modificar ese valor. En la vida real, muchas veces una persona puede usar un objeto ajeno sin ser el dueño total del mismo. Por ejemplo, puedes mirar un libro prestado por un amigo, pero no puedes escribirle encima si tu amigo también quiere usarlo. Rust refleja esta idea con préstamos.

Un préstamo es una referencia temporal a un valor que aún pertenece a otra variable. Se usa el símbolo `&` para crear una referencia inmutable y `&mut` para crear una referencia mutable. La gran regla es que si tienes una referencia mutable, no puedes tener otras referencias activas al mismo valor al mismo tiempo. Eso evita inconsistencias y carreras de datos.

Esto es clave porque permite reutilizar información sin destruir la propiedad original. El programa puede leer varios valores a la vez si son inmutables, pero si necesita modificarlos, exige exclusividad. Esa idea se ve en muchos contextos de software: si dos partes del programa pueden cambiar un mismo dato al mismo tiempo, el resultado puede volverse impredecible. Rust fuerza una ordenación clara de acceso para evitar esto.

## 3. Temas de la Sesión
- Referencias inmutables con `&T`
- Referencias mutables con `&mut T`
- Préstamos temporales y alcance
- Exclusividad de acceso para evitar conflictos

## 4. Código de Explicación
```rust
let texto = String::from("hola");
let r1 = &texto;
let r2 = &texto;
println!("{} {}", r1, r2);
```

La variable `texto` sigue siendo dueña del valor. Se crean dos referencias inmutables con `&texto`. Como ambas son solo de lectura, Rust permite que coexistan al mismo tiempo. La línea `println!` puede imprimir ambos valores sin problemas porque no hay ninguna modificación conflictiva. El valor original sigue protegido por la propiedad de `texto`.

```rust
let mut texto = String::from("hola");
let r1 = &mut texto;
println!("{}", r1);
```

Aquí se usa `&mut texto` porque se quiere mutable access. Esa referencia tiene permisos para cambiar el valor. Rust exige que no haya otra referencia activa del mismo dato mientras `r1` exista. Esto impide que dos partes del programa intenten modificar el mismo contenido simultáneamente. La idea es simple: si se escribe, hay que hacerlo de forma controlada y exclusiva.

## 5. Tabla Comparativa
| Tipo de referencia | Permite leer | Permite escribir | Puede coexistir con otras |
| --- | --- | --- | --- |
| `&T` | Sí | No | Sí, si son inmutables |
| `&mut T` | Sí | Sí | No, debe ser la única activa |
| Valor propio | Sí | Sí | Depende del contexto |

## 6. LABORATORIO 
*Como:* estudiante que ya entiende propiedad
*Quiero:* practicar la lectura y escritura compartida de un valor sin perder el control de ownership
*Para:* visualizar cómo Rust evita lecturas y escrituras conflictivas en el mismo dato

## 7. PRÁCTICA - Instrucciones para Classroom
En este ejercicio deben identificar en un programa cuáles variables se prestan como lectura y cuáles se prestan como escritura. Deben distinguir cuándo una referencia es válida y cuándo el compilador la rechaza por conflicto.

Deberán:
- Reconocer diferencias entre `&T` y `&mut T`.
- Determinar si se pueden coexistir varias referencias.
- Explicar por qué una referencia mutable exige exclusividad.
- Comparar el comportamiento de las referencias con el de un valor propio.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Diferencié préstamo y propiedad.
- [ ] Reconocí referencias inmutables y mutables.
- [ ] Validé que no haya conflictos de acceso.
- [ ] Explicité por qué Rust protege la integridad del dato.

## 9. Pregunta de Cierre
Si dos partes del programa quieren leer el mismo dato al mismo tiempo, ¿qué pasa si una de ellas necesita escribirlo? ¿Por qué Rust toma esa decisión?