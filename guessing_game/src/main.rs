use std::io;

fn main () {
    println!("Adivina el número - RUST Project!");

    println!("Escribí un número.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Ocurrió un error al leer la línea");

    println!("El número que ingresaste es el {}", guess);
}


