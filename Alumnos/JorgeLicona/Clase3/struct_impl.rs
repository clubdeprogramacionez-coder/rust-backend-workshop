// 1. Crear Estructura
struct Multa {
    fecha: String,
    motivo: String,
    placa: String,
    no_licencia: u64,
    monto: u16,
}

// 2. Implementación
impl Multa {
    // Función asociada: crea y retorna el struct
    fn nueva(fecha: String, motivo: String, placa: String, no_licencia: u64, monto: u16) -> Self {
        Self {
            fecha,
            motivo,
            placa,
            no_licencia,
            monto,
        }
    }

    fn consultarmonto(&self) -> u16 {
        self.monto
    }

    fn detalles(&self) {
        println!("[{}] Placa: {} | Motivo: {} | Monto: ${}",self.fecha, self.placa, self.motivo, self.monto);
    }
}

pub fn main() {
    let mut infraccion = Multa::nueva(
        String::from("2026-10-20"),
        String::from("Exceso de velocidad"),
        String::from("XYZ-789"),
        12345678,
        1500,
    );

    println!("--- Datos iniciales ---");
    infraccion.detalles();

    // Modificando los datos
    infraccion.motivo.push_str(" + Vuelta prohibida");
    infraccion.monto += 1200;

    println!("\n--- Datos tras la modificación ---");
    infraccion.detalles();
    println!("Monto a pagar consultado: ${}", infraccion.consultarmonto());
}

/*Representar una entidad mediante un struct en lugar de variables independientes es útil porque permite agrupar datos relacionados
bajo una sola estructura con nombre y sentido lógico, evitando que la información quede dispersa por todo el programa.

Legibilidad: El código refleja de forma más natural el problema real que se está resolviendo, ya que tratamos con
una entidad completa (como un producto o una persona) y no con datos sueltos.

Orden: Si necesitamos pasar la información a una función o crear múltiples registros, manejamos una sola unidad
estructurada en vez de decenas de variables individuales.*/