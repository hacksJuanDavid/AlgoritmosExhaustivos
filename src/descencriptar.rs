
// Funcion de fuerza bruta caracter para descencriptar el algoritmo de cesar, Total Operaciones elementales: 11
//t(n) = O(1)
fn descencriptar_fuerza_bruta_caracter(caracter: char, desplazamiento: u8) -> char {
    let mut caracter_descifrado = caracter as u8; // Convertimos el caracter a u8 para poder operar con el , Operaciones elementales: Declara, Asigna , Invocacion : 3
    caracter_descifrado = caracter_descifrado.wrapping_sub(desplazamiento); // Restamos el desplazamiento al caracter, Operaciones elementales: Asigna , Invocacion : 2
    if caracter_descifrado < 32 { // Si el caracter descifrado es menor que 32, sumamos 95 para que vuelva a estar en el rango de caracteres imprimibles, Operaciones elementales: Asigna, Aritmetica : 2
        caracter_descifrado = caracter_descifrado.wrapping_add(95); // 126 - 32 = 95 , Operaciones elementales: Asigna, Invocacion : 2
    }
    caracter_descifrado as char // Convertimos el caracter descifrado a char para poder devolverlo, Operaciones elementales: Asigna, Invocacion : 2
}

// Funcion de fuerza bruta para descencriptar el algoritmo de cesar , Total Operaciones elementales: 16
//t(n) = O(n)
//t(n) = O(n)
fn descencriptacion_fuerza_bruta(mensaje_encriptado: &str) { 
    for desplazamiento in 1..100 { // Recorremos los 100 posibles desplazamientos, Operaciones elementales: Asigna, Invocacion, Iteracion : 3
        let mut mensaje_descifrado = String::new(); // Declaramos el mensaje descifrado, Operaciones elementales: Declara, Asigna, Invoca : 3
        for caracter in mensaje_encriptado.chars() { // Recorremos el mensaje encriptado caracter a caracter, Operaciones elementales: Asigna, Invocacion, Iteracion : 3
            let caracter_descifrado = descencriptar_fuerza_bruta_caracter(caracter, desplazamiento); // Descencriptamos el caracter, Operaciones elementales: Declara, Asigna, Invoca : 3
            mensaje_descifrado.push(caracter_descifrado); // Añadimos el caracter descifrado al mensaje descifrado, Operaciones elementales: Asigna, Invoca : 2
        }
        // Imprmir los resultados de color azul en la consola
        println!("\x1b[1;34mDesplazamiento {}: {}\x1b[0m", desplazamiento, mensaje_descifrado.as_str()); // Imprimimos el mensaje descifrado, Operaciones elementales: Asigna, Invoca : 2
        //println!("Desplazamiento {}: {}", desplazamiento, mensaje_descifrado.as_str()); // Imprimimos el mensaje descifrado, Operaciones elementales: Asigna, Invoca : 2
    }
}


// Interfaz algoritmo de fuerza bruta
pub fn interfaz_fuerza_bruta() {
    println!("\n");
    // Imprimir en consola de color verde un el titulo Algoritmo de fuerza bruta para descencriptar el algoritmo de cesar tiene un limite de 100 accesos:
    println!("\x1b[1;32mAlgoritmo de fuerza bruta para descencriptar el algoritmo de cesar tiene un limite de 100 accesos:\x1b[0m");
    // Pedir mensaje a desencriptar y el mensaje sea de color amarillo
    println!("\x1b[1;33mIntroduce el mensaje a desencriptar:\x1b[0m");
    let mut mensaje = String::new();
    std::io::stdin().read_line(&mut mensaje).expect("Error al leer el mensaje");
    println!("\n");

    // Algoritmo de fuerza bruta
    descencriptacion_fuerza_bruta(mensaje.as_str());
    println!("\n");
}


