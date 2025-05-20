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

    {
        // bloco interno
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

pub fn tipo_escalar_numerico() {
    println!("\nInicio do programa");
    /* Tipos de Dados: Tipos Escalares		[3.2. Data Types: Scalar Types]

    Baseado em:
    The Rust Programming Language
    by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
    This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
    https://doc.rust-lang.org/stable/book/

    */

    // Velocidade máxima de qualquer veículo em metros por segundo
    const _VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);

    // Comprimento máximo de qualquer veículo em metros
    // const COMPRIMENTO_MAXIMO = 22;

    /*
    Table 3-1: Integer Types in Rust

    Length	Signed	Unsigned
    8-bit	i8		u8
    16-bit	i16		u16
    32-bit	i32		u32
    64-bit	i64		u64
    128-bit	i128	u128
    arch	isize	usize

    Obs1: i32 é default para inteiros
    Obs2: Em caso de overflow temos "panico na execução" (debug mode) ou "dá a volta" (release mode)
    Obs3: Existem vários métodos na biblioteca padrão para lidar com overflow
    Obs4: Existem dois tipos de ponto flutuante: f32 e f64 (default)
    */

    /*
    Table 3-2: Integer Literals in Rust

    Number literals		Example
    Decimal				98_222
    Hex					0xff
    Octal				0o77
    Binary				0b1111_0000
    Byte (u8 only)		b'A'

    Obs1: Ponto flutuante aceita  7.6e-2  ou  0.076
    */

    let _chassi: i32 = 123456; // identificação de um carro
    let _acel_max: f64 = 3.0; // metros por segundo ao quadrado
    let _acel_min: f64 = -10.0; // metros por segundo ao quadrado
    //let vel_max: f32 = VELOCIDADE_MAXIMA;	//  as f32;	// metros por segundo
    let comprimento: i32 = 4; // metros
    let posicao_atual: f32 = -100.0; // metros do cruzamento
    let vel_atual: f64 = 0.0; // metros por segundo
    let acel_atual: f64 = 0.0; // metros por segundo ao quadrado

    // adição
    let sum = posicao_atual + 10.0;

    // subtração
    let difference = vel_atual - 4.3;

    // multiplicação
    let product = comprimento * 2; // pode 2.0 ???

    // divisão
    let quotient = acel_atual / 2.0;
    let floored = 2 / 3; // truncado

    // resto da divisão
    let remainder = 43 % 5;

    println!(
        "sum: {sum}, diff: {difference}, prod: {product}, quotient: {quotient}, floored: {floored}, remainder: {remainder}"
    );

    // transformação de tipos
    let xxx: f64 = 123.55;

    //let yyy = xxx + 88;
    //let yyy = xxx + 88f64;
    //let yyy = xxx + 88 as f64;

    //https://doc.rust-lang.org/std/primitive.f64.html
    println!(
        "trunc {}, round {}, ceil {}, floor {}",
        xxx.trunc(),
        xxx.round(),
        xxx.ceil(),
        xxx.floor()
    );

    println!("Alô numéricos!");
}

pub fn tipo_escalar_bool_char() {
    let t = true;
    let f = false;

    let x = t && f;
    let _y = t || !f;
    let _z = 12 > 13;

    let c = 'z';
    let _c = 'z'; // sublinha elimina os warning
    let _z: char = 'ℤ';

    println!("bool: {x}, char {c}");
}

pub fn tipo_composto_tupla() {
    println!("\nInicio do programa");
    let tup1: (i32, f64, bool) = (500, 6.4, true);
    let tup2 = (500, 6.5, 'z');

    println!("Minha tupla tem 1:{tup1:?}, 2:{tup2:?}");

    // desestruturação (destructuring) quebra a tupla em suas partes
    let (a, b, c) = tup1;
    println!("Minha tupla tem {a} - {b} - {c}");

    // Pode acessar os campos usando indexadores
    println!("Minha tupla tem: {} - {} - {}", tup1.0, tup1.1, tup1.2);

    //Tupla vazia é chamada unit, representa um valor vazio
    println!("Tupla vazia: {:?}", ());
}

pub fn tipo_composto_array() {
    println!("\nInicio do programa");
    let _aa = [1, 2, 3, 4, 5, 6];
    let meses = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];

    let _bb: [i32; 5] = [1, 2, 3, 4, 5];
    let cc = [3; 5];
    let dd = [3, 5];

    println!("cc {cc:?}");
    println!("dd {dd:?}");

    println!("Elemento 2 do array 'meses' é: {:?}", meses[2]);

    //let errado = cc[11]; // Erro de compilação, Pânico detectado na execução!
    //println!("{errado:?}");
}

/** FUNCTIONS */
mod functions {
    pub fn outra_function() {
        println!("Outra função");
    }

    pub fn outra_function_parametros(x: i32) {
        println!("\nOutra função com parametro {x}");
    }

    pub fn print_labeled_measurement(valor: f64, unidade: char) {
        println!("A medida é: {valor}{unidade}");
    }

    pub fn soma(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn somaret(x: i32, y: i32) -> i32 {
        return x + y;
    }
}

pub fn functions() {
    println!("\nInicio do programa");
    functions::outra_function();
    functions::outra_function_parametros(10);
    functions::print_labeled_measurement(2.5, 'm');

    println!("A soma {}", functions::soma(20, 20));
    println!("A soma {}", functions::somaret(200, 150));

    let y = {
        let x = 3;
        x + 1
    };

    println!("sem ponto e virgula: {y}");

    /*let y: () = {
        let x = 3;
        x + 19999;
    }; */

    //println!("sem ponto e virgula: {y}");
}

// Controle de Fluxo

pub fn controle_de_fluxo() {
    println!("\nInicio do programa");

    let number = 3;
    //Condições deve ser do tipo bool, não precisa de parênteses
    if number < 5 {
        println!("Condição verdadeira");
    } else {
        println!("Condição falsa");
    }

    // Cascata de ifs
    if number % 4 == 0 {
        println!("número é divisivel por 4");
    } else if number % 3 == 0 {
        println!("número é divisivel por 3");
    } else if number % 2 == 0 {
        println!("número é divisivel por 2");
    } else {
        println!("número não é divisivel por 4, 3 ou 2");
    }

    //Pode usar como expressão
    let outro_number = if number == 0 { 0 } else { 99 };
    println!("O valor do outro_number é: {outro_number}");

}

pub fn controle_de_fluxo_repeticao() {
    println!("\nInicio do programa");

    let mut number = 5;
    println!("\n    Usando while");
    while number != 0 {
        println!("while {number}");
        number = number - 1;
    }

    println!("\n    Usando for");
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    for e in arr {
        println!("for {e}");        
    }

    println!("\n    Usando Range");
    for num in 1..=3 {
        println!("range {num}")
    }

    println!("\n    Usando Range Reverso");
    for num in (1..=3).rev() {
        println!("range rev {num}")
    }

}

pub fn controle_de_fluxo_repeticao_loop() {
    println!("\nInicio do programa");
    let mut i = 0;

    println!("\nloop: ");
    loop {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        println!("i {i}");
        if i >= 10 {
            break;
        }
    }

    // loop como expressão
    println!("\nloop com expressão: ");
    let result = loop {
        i += 100;
        if i >= 100 {
            break i * 2;
        }
    };
    println!("\nResult: {result}");

    //Labels em loops
    println!("\nLabels em loops");
    let mut contagem = 0;

    'loop_externo: loop {
        println!("contagem = {contagem}");
        let mut faltam = 100;

        loop {
            println!("faltam = {faltam}");
            if faltam == 97 {
                break;
            }

            println!("contagem = {contagem}");
            if contagem == 2 {
                break 'loop_externo;
            }

            faltam -= 1;
        }

        println!("Incrementa contagem");
        contagem += 1;
    }

    println!("Contagem final = {contagem}");

}