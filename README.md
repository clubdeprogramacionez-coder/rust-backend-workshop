# Workshop: Rust Server Fundamentals

Taller de introducción a Rust y desarrollo de servidores web desde cero, 
impartido por: **Cristian Beltran** Instructor del **Club de Programación de ESIME Zacatenco (IPN)**.

##  Objetivo del Taller

Aprenderás la sintaxis fundamental de Rust, su modelo de gestión de memoria (*ownership* y *borrowing*), concurrencia y cómo construir un servidor HTTP multihilo desde cero utilizando únicamente la librería estándar.

##  Módulos del Curso


##  Requisitos Previos

Tener instalado Rust y Cargo en tu sistema antes de iniciar la práctica:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
rustc --version
```
# Rust Lab 
## Seguridad de memoria garantizada en tiempo de compilación, sin manejo manual explícito.

---

### Indice

1.  [¿Por qué este laboratorio?](#1-por-que-este-laboratorio)
2.  [Instalación](#2-instalacion)
3.  [Sintaxis base de Rust](#3-sintaxis-base-de-rust)
4.  [Estructura del proyecto](#4-estructura-del-proyecto)
5.  [Plan de 16 clases](#5-plan-de-16-clases)
6.  [Cómo ejecutar](#6-como-ejecutar)

---



### 1. Instalación de Rust y Cargo

Para instalar Rust y Cargo en tu sistema, usa `rustup`:

```bash
# Windows (PowerShell)
winget install Rustlang.Rustup

# o desde la terminal oficial
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Luego verifica que queden instalados:

```bash
rustc --version
cargo --version
```

Si no están en el PATH, cierra y vuelve a abrir la terminal o reinicia VS Code.

### 2. Cómo ejecutar  proyecto

Desde la raíz del repositorio:

```bash
cargo build       # Compilar
cargo run         # Compilar y ejecutar
cargo check       # Revisar errores sin generar el ejecutable
cargo test        # Ejecutar tests
cargo run --release  # Ejecutar versión optimizada

```

También puedes compilar y ejecutar directamente:

```bash
cargo run --bin rust-backend-workshop
```

### 3. Sintaxis base de Rust
```rust
#### Variables y mutabilidad
let x = 5; // inmutable por defecto
let mut y = 10; // mutable explícita
```
#### Tipos básicos
```rust
let a: i32 = 42;
let b: f64 = 3.14;
let c: bool = true;
let d: char = 'R';
let e: &str = "stack str";
let f: String = String::from("heap string");
```
#### Funciones
```rust
fn suma(a: i32, b: i32) -> i32 {
    a + b // sin ; es retorno
}
```
#### Condicionales y bucles`
```rust
if x > 5 { } else { }

loop { break; }
while x < 10 { }
for i in 0..5 { }
```
#### Ownership - Regla base
```rust
let s1 = String::from("hola");
let s2 = s1; // s1 ya no es válido, propiedad movida
// println!("{}", s1); // error en compilacion
```
#### Borrowing
```rust
fn longitud(s: &String) -> usize {
    s.len()
}
let s = String::from("hola");
let len = longitud(&s); // préstamo, s sigue válido
```
#### Structs y Enums
```rust
struct Usuario {
    nombre: String,
    edad: u32,
}

enum Mensaje {
    Salir,
    Mover { x: i32, y: i32 },
    Escribir(String),
}
```
#### Manejo de errores sin excepciones
```rust
let r: Option<i32> = Some(5);
let r2: Result<i32, &str> = Ok(10);
```
#### Colecciones y Heap
```rust
let v = vec![1, 2, 3]; // Vec va al heap
let b = Box::new(42); // Box fuerza heap
```
#### Macros base
```rust
println!("valor: {}", x);
format!("hola {}", nombre);
```
### 3. Estructura del proyecto
```
src/main.rs -> orquestador
labs/mod.rs -> registro de clases
labs/clase01.rs -> laboratorio Clase 01
docs/ -> apuntes de cada clase
Todo lo que está en src/ compila. Nada manual fuera de cargo.
```
### 4. Plan de 16 clases

1.  ¿Por qué Rust? Stack vs Heap
2.  Ownership
3.  Borrowing y referencias
4.  Lifetimes
5.  Structs
6.  Enums y match
7.  Option y Result
8.  Vec, String, HashMap
9.  Módulos
10. Traits y genéricos
11. Closures e iteradores
12. Box, Rc, Arc
13. Concurrencia
14. Testing con cargo test
15. Proyecto I
16. Proyecto II
---

> Laboratorio: cada clase imprime direcciones de memoria con {:p} para evidenciar stack vs heap.
> Práctica: función practica() vacía y compilable dentro de cada clase.

