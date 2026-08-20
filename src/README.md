# 🦀 Rust Backend Workshop

Proyecto de aprendizaje de **Rust** mediante pequeños laboratorios.

##  Estructura

```text
src/
├── main.rs
└── labs/
    ├── mod.rs
    ├── 1-lab-stack-heap.rs
    ├── 2.lab-ownership.rs
    ├── 3-lab-borrowing.rs
    └── ...
```

##  Ejecución

Los laboratorios se ejecutan desde `main.rs`.

```rust
mod labs;

fn main() {
    println!("=== Rust Backend Workshop ===");

    labs::lab_1_stack_heap::run();
}
```

Ejecutar el proyecto:

```bash
cargo run
```

Comprobar que compile:

```bash
cargo check
```

##  Laboratorios

1. Stack y Heap
2. Ownership
3. Borrowing
4. Slices
5. Structs e `impl`
6. Enums y `match`
7. Option
8. Result
9. Traits
10. Traits estándar
11. Genéricos
12. Dyn Trait
13. TCP Echo Server
14. HTTP Parser
15. Concurrencia y Threads

Cada laboratorio cuenta con una función `run()` que puede ejecutarse desde `main.rs`.


```rust
labs::lab_1_stack_heap::run();
```
