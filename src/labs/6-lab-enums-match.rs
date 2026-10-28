//Como se crea un enum
//
/*
enum Meses {
    Enero,
    Febrero,
    Marzo,
    Noviembre,
    Diciembre,
}

enum Mensaje {
    Texto(String),
    Voz,
    TextoRemitente { msj: String, user: String },
}
*/
/*
let msj: Mensaje = Mensaje::Voz;
let msj2: Mensaje = Mensaje::Texto("Hola como estas".to_string());

let msj3 = Mensaje::TextoRemitente {
    msj: "Hola".to_string(),
    user: "Pedro".to_string(),
};

match msj3 {
    Mensaje::Voz => println!("Tienes un nuevo mensaje de voz"),
    Mensaje::Texto(mensaje) => println!("Tienes un nuevo mensaje: {mensaje}"),
    Mensaje::TextoRemitente { msj, user } => println!("{user}: {msj}"),
}


let x: Option<u8> = regresa_numero();

fn regresa_numero() -> Option<u8> {
    None
}

match x {
    Some(n) => println!("El numero es: {n}"),
    None => (),
}

let numero = "6";

let numero_convertido: Result<u32, _> = numero.parse();

match numero_convertido {
    Ok(numero) => println!("El numero convertido es: {numero}"),
    Err(e) => println!("{e}"),
}
*/
enum ErrorApi {
    ErrorDeConexion,
    TokenInvalido,
}

fn conectar_db(url: &str) -> Result<String, ErrorApi> {
    if url != "postgres://localhost" {
        return Err(ErrorApi::ErrorDeConexion);
    }
    Ok("Conexion exitosa".to_string())
}

fn validar_token(token: &str) -> Result<String, ErrorApi> {
    if token != "admin123" {
        return Err(ErrorApi::TokenInvalido);
    }

    Ok(" Datos del perfil de usuario".to_string())
}

// Esta función simula lo que hace tu controlador/ruta en el backend
fn endpoint_login(url: &str, token: &str) -> Result<String, ErrorApi> {
    // TODO: Llama a conectar_db usando '?'
    // TODO: Llama a validar_token usando '?' y devuelve su resultado
    let conexion = conectar_db(url)?;
    let token = validar_token(token)?;

    let response = conexion + &token;

    Ok(response)
}

pub fn run() {
    // Prueba cambiar estos valores para simular peticiones correctas o hackeos
    let peticion = endpoint_login("postgres://localhost", "admin123");

    match peticion {
        Ok(datos) => println!("{datos}"),
        Err(ErrorApi::ErrorDeConexion) => println!("Error al conectar la base de datos"),
        Err(ErrorApi::TokenInvalido) => println!("Credenciales incorrectas"),
    }
    // TODO: Usa un 'match' para manejar la 'peticion'
    // - Si es Ok(datos) -> Imprime los datos del usuario (Status 200)
    // - Si es Err(ErrorApi::ErrorDeConexion) -> Imprime un error interno (Status 500)
    // - Si es Err(ErrorApi::TokenInvalido) -> Imprime un error de acceso (Status 401)
}
