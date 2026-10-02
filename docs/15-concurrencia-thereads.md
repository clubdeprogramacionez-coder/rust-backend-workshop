# Sesión 15: Concurrencia y threads

## 1. Objetivo de Aprendizaje
Comprender qué es la concurrencia en Rust, cómo se crean threads y por qué la programación concurrente exige cuidado con el acceso a datos compartidos.

## 2. Lectura Guiada 
Un programa puede hacer varias cosas al mismo tiempo. Esto se llama concurrencia. En un servidor, por ejemplo, un thread puede escuchar conexiones mientras otro procesa datos. En un sistema más complejo, distintas tareas pueden ejecutarse en paralelo y compartir recursos.

Rust ofrece threads en la biblioteca estándar para ejecutar código simultáneamente. Sin embargo, el lenguaje toma muy en serio el acceso a memoria compartida. Si dos threads trabajan sobre la misma variable sin control, se pueden producir condiciones de carrera o datos inconsistentes.

Por eso Rust exige pensar cuidadosamente sobre la propiedad y el movimiento de datos entre threads. El lenguaje ayuda a prevenir errores comunes, especialmente cuando el mismo valor debe ser leído o escrito desde varias tareas a la vez.

## 3. Temas de la Sesión
- Threads
- Concurrencia y paralelismo
- Movimiento de datos entre hilos
- Seguridad en acceso compartido

## 4. Código de Explicación
```rust
use std::thread;

let manejador = thread::spawn(|| {
    println!("Hola desde un thread");
});

manejador.join().unwrap();
```

`thread::spawn` crea un nuevo hilo de ejecución. El bloque de código se ejecuta en paralelo, y `join` espera a que termine antes de continuar. Esto es útil cuando la tarea secundaria no tiene que bloquear la aplicación principal.

```rust
let mensaje = String::from("Rust");

let manejador = thread::spawn(move || {
    println!("{}", mensaje);
});
```

Aquí aparece un punto clave: el valor `mensaje` se mueve al closure del thread. Esto es necesario porque cada thread puede tener su propia propiedad de datos. El compilador ayuda a asegurar que no haya accesos inválidos a memoria compartida.

```rust
let contador = std::sync::Arc::new(std::sync::Mutex::new(0));
```

Cuando varios threads necesitan compartir un valor mutable, se suelen usar estructuras de sincronización como `Arc` y `Mutex`. `Arc` permite compartir propiedad entre múltiples threads, y `Mutex` asegura que solo un thread pueda modificar el dato a la vez.

## 5. Tabla Comparativa
| Concepto | Descripción | Riesgo principal |
| --- | --- | --- |
| Thread | Tarea concurrente | Interacciones complejas |
| `move` | Traslada propiedad al thread | Evita uso inválido |
| `Arc` | Compartición segura | Requiere sincronización |
| `Mutex` | Exclusión mutua | Evita carrera de datos |

## 6. LABORATORIO 
*Como:* estudiante que quiere entender cómo un programa puede ejecutar varias tareas al mismo tiempo
*Quiero:* crear y coordinar threads simples
*Para:* preparar la base para servidores concurrentes y servicios de red

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben crear dos o más threads que ejecuten tareas simultáneas. Después deben analizar cómo se comparten datos y qué mecanismos de Rust evitan condiciones de carrera.

Deberán:
- Crear un thread con `thread::spawn`.
- Mover datos al closure si es necesario.
- Explicar qué pasa cuando varios threads deben compartir información.
- Describir cómo `Mutex` o `Arc` ayudan en esa situación.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Creé al menos un thread.
- [ ] Usé `move` para transferir propiedad.
- [ ] Reconocí la necesidad de sincronizar datos compartidos.
- [ ] Explicité por qué Rust protege el acceso concurrente.

## 9. Pregunta de Cierre
Si dos tareas ejecutan código al mismo tiempo y comparten un valor, ¿por qué Rust exige un diseño cuidadoso y herramientas como `move`, `Arc` y `Mutex` para mantener la seguridad?

