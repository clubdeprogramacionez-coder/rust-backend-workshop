//Crear estructura
/*
struct Rectangle {
    base: u32,
    altura: u32,
}


/*/Implementacion
de la estructura*/

impl Rectangle {
    //Funcion asociada
    fn new(base: u32, altura: u32) -> Self {
        Self { base, altura }
    }
    //Funcion no asociada
    fn area(&self) -> u32 {
        self.base * self.altura
    }
}
*/

//Struct en forma de tupla
//struct Vector3(f32, f32, f32);

//Struct unitario
//

//struct INT;
//
//
//Lifetimes
//
//Estructura con tiempo de vida
struct User<'a, 'b> {
    name: &'a String,
    email: &'b String,
}

pub fn run() {
    //let rectangulo = Rectangle::new(4, 6);

    //let area = rectangulo.area();

    //println!("El area del rectangulo es: {area}")
    //
    //let punto1 = Vector3(0.0, 7.8, 3.4);

    let name = String::from("Cristian");
    let email = String::from("cristian@gmail.com");

    let user = User {
        name: &name,
        email: &email,
    };

    println!("{}", user.name);
}
