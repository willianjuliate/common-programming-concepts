// A09 s02
pub fn mutabilidade() {
    println!("Inicio do programa");
    let x = 5;
    println!("O valor de x é: {x}");

    //x = 6;                // Pode ??

    let x = 66; // Pode ??
    println!("O valor de x agora é: {x}");

    let mut y = 5;
    println!("O valor de y é: {y}");
    y = 6;
    println!("O valor de y agora é: {y}");
}

// A09 s02
const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;
//const UMA_HORA_EM_SEGUNDOS = 1 * 60 * 60;         // pode? -> não pode
//const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 600;   // pode? -> não pode
pub fn constantes() {
    //const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60; // Escopo interno, pode? -> pode
    println!("Inicio do programa");
    let mut x = 5;
    println!("O valor de x é: {x}");

    x = UMA_HORA_EM_SEGUNDOS;
    println!("O valor de x é: {x}");
}