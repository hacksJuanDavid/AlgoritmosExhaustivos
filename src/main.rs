/*
TIPS PARA LOS COMENTARIOS PARA DOCUMENTAR MY CODE DIOSSSS TODO LO QUE FALTA POR HACER "THANKS GOOD , FOR THE GOOD TIMES IN LIVE CODING "
 *aaaa
 !aaaa
 ? aaaa
 TODO : aa
 @param MyParam
*/

/*
 *Crearemos un algoritmo de descencriptacion para un mensaje encriptado con el algoritmo de cesar
 *El algoritmo de cesar consiste en desplazar el abecedario un numero de veces para encriptar el mensaje
 *Para desencriptar debemos usar un algoritmo de fuerza bruta que consiste en probar todas las posibilidades hasta encontrar la correcta
 *Para ello debemos probar todas las posibilidades de desplazamiento del abecedario
 */

mod descencriptar;
mod encriptar;

use descencriptar::interfaz_fuerza_bruta;
use encriptar::interfaz_encriptar;

// Menu principal
fn menu() {
    println!("\n");
    // De color verde el texto de bienvenida
    println!("\x1b[32m{}\x1b[0m", "Bienvenido al menu de algoritmos!");
    println!("1. algoritmo de encriptacion de cesar");
    println!("2. algoritmo de fuerza bruta para descencriptar el algoritmo de cesar");
    println!("3. Salir");
    println!("\n");
    // De color amarillo el texto de elegir opcion
    println!("\x1b[33m{}\x1b[0m", "Elija una opcion:");

    let mut opcion = String::new();
    std::io::stdin().read_line(&mut opcion).expect("Error al leer la opcion");

    match opcion.trim() {
        "1" => interfaz_encriptar(),
        "2" => interfaz_fuerza_bruta(),
        "3" => std::process::exit(0),
        _ => {
            println!("Opcion no valida");
            menu();
        }
    }
}

fn main() {
    loop {
        menu();
    }
}
