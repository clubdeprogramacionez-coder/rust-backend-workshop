fn main() {
    //tipados de  u8, u16, u32, u128, usize
    // Con signo: i8, i16, i32, i64, i128, isize
    let entero: u8 = 1;//se guarda en el stack

    let _entero_n: i8 = -1;//se guarda en el stack
    
    let flotante: f32= 10.99; //se guarda en el stack

    //los flotantes no tienen tipado especifico negativo
    let _flotante_doble: f64 = -250.75;   // Doble precisión, también en el stack
    
    let logico:bool= false; //se guarda en el stack
    
    let texto: &str = "Esto es un texto str...";//se guarda en el stack
    
    let texto2: String=String::from ("Esto es un texto con String...");//se guarda en el heap
    
    let lista: [i32; 3] = [10, 20, 30];//se guarda en el stack

    // tupla y deconstruccion (se guarda en el stack)
    let mi_tupla: (u16, bool, &str) = (2026, true, "ESIME");
    let (anio, activo, escuela) = mi_tupla;
    
    println!("==================================================");
    println!("entero con tipado :u8 ===>> {entero}");
    println!("==================================================");
    println!();
    println!("==================================================");
    println!("entero negativo con tipado :i8 ===>> {_entero_n}");
    println!("==================================================");
    println!();
    println!("==================================================");
    println!("flotante con tipado :f32 ===>> {flotante}");
    println!("==================================================");
    println!();
    println!("==================================================");
    println!("flotante con tipado :f64 ===>> {_flotante_doble}");
    println!("==================================================");
    println!();
    println!("==================================================");
    println!("Logico con unico tipado : bool ===>> {logico}");
    println!("==================================================");
    println!();
    println!("==================================================");
    println!("texto con tipado :&str ===>> {texto}");
    println!("==================================================");
    println!();
    println!("==================================================");
    println!("texto con tipado :String ===>> {texto2}");
    println!("==================================================");
    println!();
    /*se usa :? para imprimir un arreglo, vector o tupla a menos que sea
    por elemento se puede dejar solo las llaves {}*/
    println!("=============================================================================");
    println!("Tupla completa: {:?}", mi_tupla);
    //println!("Tupla completa: {}", mi_tupla.0);
    println!("Tupla deconstruida ===>> anio: {anio}, activo: {activo}, escuela: {escuela}");
    println!("=============================================================================");
    println!();
    println!("==================================================");
    println!("Array fijo en Stack: {:?}", lista);
    //println!("Array fijo en Stack: {:?}", lista[2]);
    println!("==================================================");
}