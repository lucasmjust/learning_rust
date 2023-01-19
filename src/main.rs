fn main() {

    // tipos_de_dados();

    estruturas_de_controle()
}

// fn tipos_de_dados(){
//     let x = 45; // Quando não definido, o compilador infere o tipo i32 que é o tipo standed
//     let y: u64 = 32; // Inteiro não negativo com 64bits e valor 32.
//     let f: f32 = 12.45; // Float com 32bits e valor 12.45
//     let b: bool = true; // Booleanos podem receber valores true ou false

//     println!("Variável: inteiro standed e valor {x}");
//     println!("Variável: Inteiro não negativo de 64bits e valor {y}");
//     println!("Variável: Float com 32bits e valor {f}");
//     println!("Variável: Boolean e valor {b}");
// }

fn estruturas_de_controle(){
    let number1: i32 = 55;
    let number2: i32 = 15;

    if number1 > number2 {
        println!("{} > {}", number1, number2)
    }else if number1 == number2{
        println!("{} = {}", number1, number2)
    }else{
        println!("{} < {}", number1, number2)
    }
}
