// Simulación local: no consulta ni modifica servidores.
// Ejecuta: cargo run --example estado_infraestructura
fn main() {
    let servicio = "api-inventario";
    let replicas_deseadas: u32 = 4;
    let mut replicas_disponibles: u32 = 1;

    println!("Servicio: {servicio}");
    println!("Inicial: {replicas_disponibles}/{replicas_deseadas} replicas disponibles");

    replicas_disponibles += 2;
    println!("Tras arrancar dos: {replicas_disponibles}/{replicas_deseadas}");

    replicas_disponibles -= 1;
    println!("Tras fallar una: {replicas_disponibles}/{replicas_deseadas}");

    // Fotografía del estado final, calculada después de los eventos.
    // Estos valores conocidos cumplen disponibles <= deseadas.
    let replicas_pendientes = replicas_deseadas - replicas_disponibles;
    let objetivo_cumplido = replicas_disponibles == replicas_deseadas;
    println!("Pendientes: {replicas_pendientes}");
    println!("Objetivo cumplido: {objetivo_cumplido}");
}
