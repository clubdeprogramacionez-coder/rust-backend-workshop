# Sesión 13: TCP Echo Server

## 1. Objetivo de Aprendizaje
Comprender cómo funciona la comunicación de bajo nivel entre procesos usando TCP/IP, y cómo un servidor puede escuchar conexiones, recibir datos y devolver exactamente lo mismo que recibió como base para sistemas backend, B2B y P2P.

## 2. Lectura Guiada 
TCP y IP son dos piezas fundamentales de la comunicación de red. IP se encarga del envío de paquetes entre dispositivos, indicando desde dónde sale y hacia dónde llega. TCP, por su parte, agrega una capa más confiable: ordena los datos, confirma que llegaron y reintenta la entrega si algo se pierde. Es decir, IP resuelve “dónde va el paquete”, mientras TCP resuelve “cómo se garantiza que el mensaje llegue completo y en orden”.

Cuando hablamos de comunicación B2B o P2P, no siempre estamos enviando peticiones HTTP o usando gRPC. Muchas veces dos servicios deben hablar directamente a nivel de socket, sin una capa de aplicación intermedia. Un servidor TCP abre un puerto y escucha conexiones entrantes. Un cliente se conecta a ese puerto, envía información y espera la respuesta. En un echo server, el servidor recibe el mensaje y lo devuelve exactamente igual. Esa simple operación es muy útil para aprender el flujo real de la red: conectar, leer, procesar y responder.

Este tipo de comunicación es más baja en nivel que HTTP, porque no hay reglas semánticas de recursos, verbos ni JSON. Solo hay bytes que pasan por una conexión establecida. Eso hace que TCP sea perfecto para sistemas que necesitan velocidad, control y comunicación directa entre procesos, especialmente cuando dos máquinas o servicios se conectan sin una capa web por encima.

## 3. Temas de la Sesión
- IP como enrutamiento de paquetes
- TCP como flujo confiable y ordenado
- Sockets y puertos
- Echo server como ejemplo de comunicación directa B2B/P2P

## 4. Código de Explicación
```rust
let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
```

Aquí se crea un `TcpListener`, que es la parte del servidor que “escucha” conexiones entrantes en una dirección y puerto. `127.0.0.1:8080` significa que el servidor va a aceptar conexiones locales en el puerto 8080. Este es el primer paso para abrir una comunicación de red real: preparar el punto de entrada de la conexión.

```rust
for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();
    stream.write_all(&buffer[..n]).unwrap();
}
```

El bucle `for stream in listener.incoming()` acepta cada conexión entrante. Cada `stream` representa un canal TCP entre cliente y servidor. Luego se crea un buffer para leer los datos recibidos, y `read` toma los bytes enviados por el cliente. Finalmente, `write_all` reenvía exactamente esos bytes al cliente. Esa es la lógica del echo server: recibir y devolver el mismo mensaje.

Este flujo es muy importante porque refleja cómo funciona TCP en la práctica: se conecta, llega información, se procesa y se da una respuesta con el mismo canal. En sistemas más complejos, la respuesta puede ser un JSON, un comando, un resultado de cálculo o un mensaje de control, pero la base sigue siendo la misma.

## 5. Tabla Comparativa
| Capa / Concepto | Descripción | Ejemplo de uso |
| --- | --- | --- |
| IP | Enruta paquetes entre dispositivos | Dirección origen y destino |
| TCP | Garantiza entrega, orden y conexión | Sockets y flujo confiable |
| HTTP | Protocolo de aplicación sobre TCP | APIs REST |
| gRPC | RPC sobre HTTP/2 | Microservicios modernos |
| TCP echo server | Comunicación directa en socket | B2B/P2P, servicios internos |

## 6. LABORATORIO 
*Como:* estudiante que quiere entender redes de software a nivel real
*Quiero:* levantar un servidor TCP que reciba y responda mensajes
*Para:* visualizar cómo dos servicios se comunican directamente a bajo nivel, sin depender de HTTP ni de una capa de aplicación más alta

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben crear un servidor TCP simple que escuche conexiones entrantes, reciba un mensaje desde un cliente y lo devuelva exactamente como llegó. La idea es observar el flujo real de la comunicación de red en una conexión confiable.

Deberán:
- Definir una dirección y un puerto para escuchar.
- Aceptar conexiones entrantes con un socket TCP.
- Leer bytes del cliente con un buffer.
- Escribir la misma información de vuelta como respuesta.
- Explicar la diferencia entre esta comunicación y una llamada HTTP o gRPC.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Configuré un socket TCP.
- [ ] Escuché conexiones en un puerto.
- [ ] Leí datos del cliente.
- [ ] Reenvié el mismo mensaje como respuesta.
- [ ] Relacioné la solución con comunicación B2B/P2P de bajo nivel.

## 9. Pregunta de Cierre
Si dos servicios necesitan intercambiar mensajes de forma directa y confiable, ¿por qué TCP es una opción más cercana a la comunicación de red real que HTTP o gRPC, y qué ventajas aporta cuando se habla de sistemas B2B o P2P?
