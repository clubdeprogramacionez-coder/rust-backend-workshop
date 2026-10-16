fn main(){
    let x=1;
    let y=x;
    println!("x: {x} | y: {y}"); //Complentamente legal al tener almacenadas en el stack
                                 //esto crea dos valores independientes en memoria y cada variable es dueña
                                 //de su propio valor

    let s1 = String::from("Laboratorio");
    /*
        let s2 = s1; 
        println!("{}", s1); // Esto generaría un error de compilación
        
        El tipo String es dinámico y su contenido se almacena en el Heap.
        Dado que Rust tiene la regla estricta de que solo puede haber un
        dueño a la vez para cualquier porción de memoria, al declarar 
        
        let s2 = s1;
        
        la propiedad del texto "Laboratorio" se transfiere de s1 a s2.

        - s2 se convierte en el nuevo y único dueño del dato.

        - s1 queda inmediatamente invalidada y el compilador prohibirá su uso a partir de esa línea.    
    */

    {
        let s = String::from("Datos internos"); // 's' es válida desde aquí
    
        // durante este bloque s se puede usar y modificar pero una vez termine este bloque de codigo se
        // ejecuta un drop de forma automatica.
        println!("aqui si se puede usar s: {s}");
        println!("aqui si se puede usar s1: /*{s1}*/ -> invalidada");
        println!("aqui si se puede usar s2: {s2} -> valor que ahora le pertenece de s1");
    } // El scope termina aquí. 's' sale de alcance.

    {
        {
            let documento = String::from("Reporte Anual");
    
            let lector_1 = &documento;
            let lector_2 = &documento;
            let lector_3 = &documento;
    
            // Rust permite esto porque solo se presta la referencia, como solo lectura.
            println!("{} leído por Lector 1, Lector 2 y Lector 3", documento);
        }
        
        //Permite modificar el valor original sin transferir el Ownership, pero requiere exclusividad absoluta.
        {
            let mut saldo = 100; // La variable original debe ser declarada como 'mut'
    
            {
                let modificador = &mut saldo; // Pedimos la única referencia mutable permitida
                *modificador += 50; // Usamos '*' para desreferenciar y sumar 50 al valor real
            } // 'modificador' sale de scope aquí, liberando el préstamo.
    
            println!("El saldo final es: {}", saldo); // Imprime 150
        }
    }

    {
        {

            let mut texto = String::from("Hola");
            let lector = &texto; // Préstamo inmutable (ok)
            
            // let escritor = &mut texto;
            
            // no puedes pedir un mutable cuando ya se pidio uno inmutable
            // No puedes tomar prestado 'texto' como mutable porque ya está prestado como inmutable.
            // Si 'escritor' cambiara el texto a sus espaldas, 'lector' leería basura.
            
            println!("El lector dice: {}", lector);
        }
    }

}