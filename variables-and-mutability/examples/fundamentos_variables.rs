// Ejecuta: cargo run --example fundamentos_variables
const MAX_INTENTOS: u32 = 3;

fn main() {
    // Configuración fija durante esta ejecución.
    let servicio = "api";
    let puerto: u16 = 8080;
    println!("Servicio: {servicio}; puerto: {puerto}");

    // Estado que cambia; el tipo u32 permanece igual.
    let mut intentos: u32 = 0;
    println!("Intentos iniciales: {intentos}");
    intentos += 1;
    println!("Intentos actuales: {intentos}/{MAX_INTENTOS}");

    // Shadowing crea otra vinculación: texto -> longitud en bytes.
    let etiqueta = "api";
    println!("Etiqueta: {etiqueta}");
    let etiqueta = etiqueta.len();
    println!("Longitud en bytes: {etiqueta}");

    // El nombre interior existe únicamente en este bloque.
    let replicas = 3;
    {
        let replicas = 5;
        println!("Replicas dentro del bloque: {replicas}");
    }
    println!("Replicas fuera del bloque: {replicas}");
}
