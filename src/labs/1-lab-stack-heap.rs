pub fn run() {
    /*
    //Numericos
    //Numeros enteros sin signo
    //u8 u16 u32 u64 u128 usize
    //
    let edad: u8 = 30;
    //Numeros enteros con signo
    //i8 i16 i32 i64 i128 isize
    let velocidad: i8 = -40;
    //Numeros decimales
    //f8 f16 ...
    let precio_ps5:f32 = 10999.99;
    // Booleans
    let mayor_edad = true;
    let mexicano = false;

    //Strings literales
    // &str
    let nombre: &str = "Cristian";
    */

    /*
    let x = 5;

    //x += 3;   - PELIGRO
    println!("---------------------------------");
    println!("El resultado es: {x}");
    println!("---------------------------------");

    //SHADOWING

    let y = 6;
    println!("---------------------------------");
    println!("El resultado es: {y}");
    println!("---------------------------------");
    let y = y + 3;
    println!("---------------------------------");
    println!("El resultado ahora es: {y}");
    println!("---------------------------------");
    */

    /*
    //Variables mutables
    let mut suma: u16 = 0;
    println!("---------------------------------");
    println!("El resultado antes de sumar: {suma}");
    println!("---------------------------------");
    suma += 90;
    println!("---------------------------------");
    println!("El resultado despues de sumar: {suma}");
    println!("---------------------------------");
    */

    /*
    //Arreglos y Tuplas
    let arreglo = [1, 2, 3, 4, 5, 6, 76];
    println!("{}", arreglo[3]);

    let arreglo_tipado: [u8; 3] = [3, 4, 5];
    println!("{}", arreglo_tipado[2]);

    let nombres: [&str; 2] = ["pepe", "juan"];
    */

    /*
    let tupla = (3, 4, 2, "a");
    println!("{}", tupla.1);

    let tupla_tipada: (u8, f32, &str, bool) = (2, 5.5, "hola", false);
    println!("{}", tupla_tipada.1);

    //Desestructurando una tupla

    let color: (u8, u8, u8) = (255, 128, 76);
    let (r, g, b) = color;

    println!("---------------------------------");
    println!("R: {r}, G: {g}, B: {b}");
    println!("---------------------------------");
    */

    // String != &str
    //Esto se guarda en el Heap

    /*
    let mut nombre: String = String::from("Cristian");
    nombre += " Beltran";

    println!("");
    */
}
