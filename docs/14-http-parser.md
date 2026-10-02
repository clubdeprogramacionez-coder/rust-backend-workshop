# Sesión 14: HTTP Parser

## 1. Objetivo de Aprendizaje
Entender cómo funciona el análisis de una petición HTTP básica, identificando líneas, método, ruta, versión y cabeceras para construir un servidor que entienda mensajes reales del protocolo.

## 2. Lectura Guiada 
Cuando un navegador o un cliente envía una solicitud HTTP, no manda un conjunto de datos abstractos: manda texto estructurado. Una petición típica incluye:
- método (`GET`, `POST`, etc.)
- ruta (`/`, `/usuarios`, `/api`)
- versión (`HTTP/1.1`)
- cabeceras (`Host`, `Content-Type`, etc.)
- opcionalmente un cuerpo

Un parser HTTP toma ese texto y lo transforma en una estructura entendible por el programa. En Rust, esto suele hacerse con slices, cadenas y validación de formato. El objetivo es reconocer la petición y separar sus partes sin perder información ni asumir datos inválidos.

Al analizar HTTP, también aprendemos algo muy importante: los protocolos de red no son magia; están basados en reglas definidas. Leer esas reglas y convertirlas en código permite construir servicios que puedan entender peticiones reales del mundo.

## 3. Temas de la Sesión
- Petición HTTP básica
- Línea de solicitud
- Cabeceras y rutas
- Parsing manual y validación

## 4. Código de Explicación
```rust
let request = "GET /hola HTTP/1.1\r\nHost: localhost:8080\r\n\r\n";
```

La cadena representa una solicitud HTTP. La primera línea contiene el método, la ruta y la versión. Luego aparecen cabeceras separadas por saltos de línea, y finalmente hay una línea en blanco que indica el fin de las cabeceras.

```rust
let start_line = request.split("\r\n").next().unwrap();
let parts: Vec<&str> = start_line.split(' ').collect();
println!("{:?}", parts);
```

Esto separa la línea de inicio y la divide por espacios. El resultado es algo como `"GET"`, `"/hola"` y `"HTTP/1.1"`. Es una forma muy sencilla de extraer la información clave de la solicitud.

```rust
for line in request.lines() {
    println!("{}", line);
}
```

Con `lines()` se puede iterar cada línea del mensaje. Este tipo de análisis permite detectar la línea inicial, las cabeceras y las secciones del cuerpo. Es una estrategia básica y útil para herramientas de red y servidores.

## 5. Tabla Comparativa
| Parte del mensaje | Descripción | Ejemplo |
| --- | --- | --- |
| Línea de solicitud | Método, ruta y versión | `GET / HTTP/1.1` |
| Cabeceras | Metadatos del mensaje | `Host: localhost` |
| Cuerpo | Datos del mensaje | JSON o formulario |
| Separador | Delimita cabeceras y cuerpo | `\r\n\r\n` |

## 6. LABORATORIO 
*Como:* estudiante que quiere entender cómo se comunica un cliente con un servidor
*Quiero:* parsear una petición HTTP mínima
*Para:* conocer la estructura del protocolo y prepararme para construir un servidor real

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben analizar una petición HTTP simple y extraer sus partes principales. El objetivo es entender cómo llega la información desde un cliente y cómo el servidor puede identificar qué recibe.

Deberán:
- Definir una cadena con una petición HTTP.
- Separar la línea inicial y las cabeceras.
- Extraer el método, la ruta y la versión.
- Explicar la diferencia entre línea de solicitud, cabeceras y cuerpo.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Parseé una petición HTTP básica.
- [ ] Identifiqué método, ruta y versión.
- [ ] Separé cabeceras y cuerpo.
- [ ] Relacioné esto con la capa de aplicación de red.

## 9. Pregunta de Cierre
Si cada solicitud HTTP sigue una estructura ordenada y reconocible, ¿por qué es importante parsear correctamente esos datos antes de decidir cómo responder al cliente?

