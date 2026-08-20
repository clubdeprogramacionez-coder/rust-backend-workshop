# Sesión 4: Slices y Analizador

## 1. Objetivo de Aprendizaje
Comprender qué es un slice en Rust, cómo se usa para observar una parte de un arreglo o cadena sin copiar datos, y por qué la validación de límites es fundamental para la seguridad del programa.

## 2. Lectura Guiada 
Un slice es como mirar una porción de un documento sin mover ni duplicar el documento completo. Si tienes una frase entera, puedes decidir leer solo una parte: desde la palabra 2 hasta la 5, por ejemplo. En Rust, un slice representa una vista contigua de datos. No crea una copia; solo muestra una región válida dentro de un arreglo, cadena o colección.

Esto es relevante porque muchas veces un programa no necesita todo el contenido, sino una parte específica. El idioma se vuelve más seguro cuando el rango debe ser válido. Rust verifica que el índice inicial y final estén dentro de los límites del arreglo o de la cadena. Si no lo están, el compilador no deja compilar o el programa falla con un error claro y bien definido.

También se usa para analizar texto o interpretar secuencias de datos. Cuando leemos una cadena o una entrada, normalmente queremos pasos o fragmentos: una palabra, una subcadena, una sección de un arreglo. Los slices permiten eso sin copiar todos los elementos. En otras palabras, trabajan como una ventana sobre la memoria, haciendo más eficiente y seguro el trabajo con colecciones.

## 3. Temas de la Sesión
- Arreglos contiguos y acceso por índice
- Slices como vista parcial de datos
- Rangos y límites válidos
- Análisis de cadenas y secuencias sin copiar memoria

## 4. Código de Explicación
```rust
let numeros = [10, 20, 30, 40, 50];
let parte = &numeros[1..4];
println!("{:?}", parte);
```

La variable `numeros` es un arreglo con cinco elementos. El slice `&numeros[1..4]` toma una sección del arreglo: desde el índice 1 hasta el 4, sin incluir el último. Esto significa que se visualiza una parte del arreglo. No se copia el arreglo completo; se trabaja con una referencia a la región elegida. Esa referencia sigue siendo válida solo si el rango está dentro de los límites del arreglo.

```rust
let texto = String::from("programacion");
let palabra = &texto[0..5];
println!("{}", palabra);
```

Aquí se toma un rango de la cadena para obtener una subcadena. `0..5` significa desde el inicio hasta el índice 5 (exclusivo). Se observa una parte del texto, no se genera una nueva cadena separada en memoria. Rust usa el sistema de límites para asegurarse de que ese rango sea válido dentro de la estructura original. Si alguien intenta pedir un rango fuera de capacidad, el compilador o el runtime indicará la situación.

## 5. Tabla Comparativa
| Concepto | Qué representa | Beneficio |
| --- | --- | --- |
| Arreglo | La colección completa | Acceso directo por índices |
| Slice | Una vista parcial | No duplica datos |
| Rango | Límites de la sección | Evita errores de acceso |
| Cadena | Texto con validación de bytes | Seguridad al manipular texto |

## 6. LABORATORIO 
*Como:* estudiante de análisis de datos y texto
*Quiero:* crear una vista parcial de una colección y validar sus límites
*Para:* entender cómo Rust trabaja con segmentos sin romper la seguridad del programa

## 7. PRÁCTICA - Instrucciones para Classroom
En este laboratorio deben trabajar con arreglos y cadenas para extraer porciones válidas. El objetivo no es copiar datos, sino observar una sección de la colección y justificar por qué ese rango es correcto.

Deberán:
- Definir un arreglo o texto de referencia.
- Crear uno o varios slices con rangos distintos.
- Validar qué rangos son correctos y cuáles no.
- Explicar la diferencia entre un arreglo completo y una vista parcial.

Comandos de compilación y ejecución:
```bash
cargo build
cargo run
```

Checklist de Entregable
- [ ] Identifiqué un rango válido.
- [ ] Diferencié arreglo completo y slice.
- [ ] Validé límites del segmento.
- [ ] Explicité la ventaja de no copiar datos.

## 9. Pregunta de Cierre
Si un slice representa una vista de una colección, ¿qué ventaja tiene respecto a crear un nuevo arreglo con los mismos datos y qué problema evita al validar límites?