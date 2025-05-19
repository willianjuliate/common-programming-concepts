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
    println!("\nInicio do programa");
    let mut x = 5;
    println!("O valor de x é: {x}");

    x = UMA_HORA_EM_SEGUNDOS;
    println!("O valor de x é: {x}");
}

//A10 s02
pub fn bloco() {
    println!("\nInicio do programa");
    const X: i32 = 5;
    let y = 6;
    let mut z = 7;
    z = z + 1;

    println!("No início os valores são: X={X}, y={y}, z={z}");

    {                           // bloco interno
        const X: i32 = 555;
        let y = 666;
        let mut z = 777;
        z = z + 1;
        println!("No início os valores são: X={X}, y={y}, z={z}");
    }

    println!("Depois do bloco interno os valores são: X={X}, y={y}, z={z}");
}

pub fn sombreamento() {
    println!("\nInicio do programa");
    let x = 5;
    println!("O valor de x é: {x}");
    let x = x + 5;
    println!("O valor de x é: {x}");

    {
        let x = x * 2;
        println!("O valor de x no bloco interno é : {x}");
    }

    println!("O valor de x depois do bloco interno é : {x}");

    let spaces: &str = "   ";
    let spaces = spaces.len(); // let cria nova variável com novo tipo
    println!("O valor de spaces é: {spaces}");

    let mut spaces2 = "   ";
    println!("O valor de spaces2 é: {spaces2}");
    spaces2 = "qwerty"; // mesma variável com mesmo tipo
    println!("O valor de spaces2 é: {spaces2}");

}