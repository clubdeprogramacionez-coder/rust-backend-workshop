# Sesión 16: Proyecto final - Servidor HTTP

## 1. Objetivo de Aprendizaje
Construir un servidor HTTP simple con Rust, entendiendo cómo escuchar conexiones TCP, recibir peticiones del cliente y responder con un mensaje básico según la ruta o el método solicitado.

## 2. Lectura Guiada 
A lo largo del taller hemos visto memoria, ownership, enums, traits, sockets, parsing y concurrencia. Ahora todas esas ideas se reúnen en una aplicación real: un servidor HTTP pequeño que puede aceptar conexiones y responder a solicitudes.

El flujo básico de un servidor es sencillo:
1. abrir un socket y escuchar en un puerto
2. aceptar conexiones entrantes
3. leer la petición del cliente
4. analizar una línea básica de HTTP
5. responder con un mensaje apropiado

Aunque el servidor será pequeño, representa el mismo patrón que usan servicios reales: escuchar, procesar y responder. Además, permite conectar conceptos de red con programación segura y modular.

## 3. Temas de la Sesión
- Servidor TCP
- Escucha y conexión
- Parser HTTP mínimo
- Respuesta al cliente
- Proyecto integrador

## 4. Código de Explicación
```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        println!("Solicitud recibida:\n{}", request);

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n\r\nOK";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
```

Este ejemplo crea un listener en localhost:8080. Cuando llega una conexión, se lee la solicitud y se responde con una respuesta HTTP básica. No es un servidor completo de producción, pero sí demuestra la base del flujo real.

```rust
let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n\r\nOK";
```

La cadena de respuesta sigue el formato mínimo del protocolo HTTP. Primero se indica el estado, luego las cabeceras y finalmente el cuerpo. Este patrón es esencial para cualquier servidor web mínimo.

## 5. Tabla Comparativa
| Componente | Función | Ejemplo |
| --- | --- | --- |
| `TcpListener` | Escucha conexiones | `127.0.0.1:8080` |
| `TcpStream` | Representa una conexión | Canal entre cliente y servidor |
| Parser HTTP | Descompone la petición | `GET / HTTP/1.1` |
| Respuesta HTTP | Devuelve resultado | `200 OK` |

## 6. LABORATORIO 
*Como:* estudiante que ya conoce los bloques básicos del lenguaje y la red
*Quiero:* construir un servidor HTTP para procesar peticiones simples
*Para:* integrar conocimientos de Rust aplicados a un proyecto realista

## 7. PRÁCTICA - Instrucciones para Classroom
En este proyecto deben implementar un servidor pequeño que escuche peticiones HTTP en un puerto local y responda con texto simple. La intención es combinar todos los conceptos aprendidos: manejo de memoria, `Result`, `Option`, traits, parsing y sockets.

Deberán:
- Crear un `TcpListener` en un puerto local.
- Aceptar conexiones entrantes.
- Leer y mostrar la petición del cliente.
- Responder con una respuesta HTTP válida.
- Explicar cómo la lógica del servidor se relaciona con el protocolo web.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Levanté un servidor TCP local.
- [ ] Leí una petición HTTP básica.
- [ ] Respondí con un mensaje HTTP válido.
- [ ] Integré conceptos aprendidos de Rust en un proyecto realista.

## 9. Pregunta de Cierre
Si un cliente y un servidor se comunican mediante texto estructurado, ¿por qué es necesario entender tanto el protocolo HTTP como el manejo de conexiones para construir un servicio web funcional y seguro?

