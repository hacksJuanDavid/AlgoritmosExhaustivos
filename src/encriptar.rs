// Funcion encargada de encriptar un caracter con el algoritmo de cesar, Total Operaciones elementales: 8
//t(n) = O(1)
fn encriptar_caracter_cesar(caracter: char, desplazamiento: u8) -> char {
    let mut caracter_encriptado = (caracter as u8) + desplazamiento; // Convertimos el caracter a u8 para poder operar con el, Operaciones elementales: Declara, Asigna , Aritmetica, Invocacion : 4
    if caracter_encriptado > 126 {
        // Si el caracter encriptado es mayor que 126, restamos 95 para que vuelva a estar en el rango de caracteres imprimibles, Operaciones elementales: Asignacion, Aritmetica : 2
        caracter_encriptado = caracter_encriptado - 95; // 126 - 32 = 95 , Operaciones elementales: Asignacion, Aritmetica : 2
    }
    caracter_encriptado as char // Convertimos el caracter encriptado a char para poder devolverlo, Operaciones elementales:  Asigna, Invocacion : 2
}

//Algoritmo de encriptacion de cesar, Total Operaciones elementales: 12
//t(n) = O(n)
fn encriptar_cesar(mensaje: &str, desplazamiento: u8) -> String {
    let mut mensaje_encriptado = String::new(); // Creamos un string vacio para guardar el mensaje encriptado, Operaciones elementales: Declara, Asigna, Invoca : 3
    for caracter in mensaje.chars() {
        // Recorremos el mensaje caracter a caracter y lo encriptamos, Operaciones elementales: Asigna , Invoca , Iteracion : 3
        let caracter_encriptado = encriptar_caracter_cesar(caracter, desplazamiento); // Encriptamos el caracter actual, Operaciones elementales: Declara, Asigna , Invoca : 3
        mensaje_encriptado.push(caracter_encriptado); // Añadimos el caracter encriptado al mensaje encriptado , Operaciones elementales: Asigna , Invoca : 2
    }
    mensaje_encriptado // Devolvemos el mensaje encriptado , Operaciones elementales: Asigna : 1
}

// Interfaz algoritmo de encriptacion de cesar
pub fn interfaz_encriptar() {
    println!("\n");
    // Imprimir en consola de color verde un el titulo Encriptar con algoritmo de cesar tiene un limite de 100 desplazamientos
    println!("\x1b[1;32mEncriptar con algoritmo de cesar tiene un limite de 100 desplazamientos\x1b[0m");
    // Pedir mensaje a encriptar y el mensaje sea de color amarillo
    println!("\x1b[1;33mIntroduce el mensaje a encriptar: \x1b[0m"); 
    let mut mensaje = String::new();
    std::io::stdin().read_line(&mut mensaje).expect("Error al leer el mensaje");

    // Pedir desplazamiento
    println!("Introduce el desplazamiento: ");
    let mut desplazamiento = String::new();
    std::io::stdin().read_line(&mut desplazamiento).expect("Error al leer el desplazamiento");
    // Convertimos el desplazamiento a u8
    let mut desplazamiento: u8 = desplazamiento
        .trim()
        .parse()
        .expect("El desplazamiento debe ser un numero");


    // Si los desplazaientos son mayores que 100, se establecen a 100
    if desplazamiento > 100 {
        println!("El desplazamiento es mayor que 100, se establece a 100");
        desplazamiento = 100;
    }else{
        println!("Desplazamiento establecido a {}", desplazamiento);
    }
    
    //Algoritmo de encriptacion de cesar
    let mensaje_encriptado = encriptar_cesar(mensaje.as_str(), desplazamiento);
    println!("Mensaje encriptado: {}", mensaje_encriptado);
    println!("\n");
}
