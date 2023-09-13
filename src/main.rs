use std::io;
use colored::Colorize;

const MENSAGEM_DE_FALHA: &str = "Falha ao ler o valor";
const MENSAGEM_DE_FALHA_INTEIRO: &str = "Falha ao ler o valor";

fn main() {
    loop {
        println!(
            "{}",
            "\n Escolha uma opção: \n [1 - somar] \n [2 - subtrair] \n [3 - multiplicar] \n [4 - dividir] \n [0 - sair ]"
                .to_uppercase()
                .bright_blue()
        );
        let mut escolha = String::new();

        io::stdin()
            .read_line(&mut escolha)
            .expect(MENSAGEM_DE_FALHA);
        let escolha: u8 = match escolha.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match escolha {
            0 => break,
            1 => somar(),
            2 => subtrair(),
            3 => multiplicar(),
            4 => duvidir(),
            _ => {continue}
        }
    }
}

fn somar() {
    println!("{}", "Somar".to_uppercase().yellow());
    let (primeiro_valor, segundo_valor) = input_valores();

    println!("{}", "RESULTADO:".green());
    print!( "{} + {} = {}\n", primeiro_valor, segundo_valor, primeiro_valor + segundo_valor);
}

fn subtrair() {
    println!("{}", "subtrair".to_uppercase().yellow());
    let (primeiro_valor, segundo_valor) = input_valores();

    println!("{}", "RESULTADO:".green());
    print!( "{} - {} = {}\n", primeiro_valor, segundo_valor, primeiro_valor - segundo_valor);
}

fn multiplicar() {
    println!("{}", "Multiplicar".to_uppercase().yellow());
    let (primeiro_valor, segundo_valor) = input_valores();

    println!("{}", "RESULTADO:".green());
    print!( "{} * {} = {}\n", primeiro_valor, segundo_valor, primeiro_valor * segundo_valor);
}

fn duvidir() {
    println!("{}", "Dividir".to_uppercase().yellow());

    let (primeiro_valor, segundo_valor) = input_valores();

    println!("{}", "RESULTADO:".green());
    print!("{} / {} = {}\n", primeiro_valor, segundo_valor, primeiro_valor / segundo_valor);
}

fn input_valores() -> (i32, i32) {
    println!("{}", "Digite o primeiro valor a ser somado.".cyan());
    let mut primeiro_valor = String::new();

    io::stdin()
        .read_line(&mut primeiro_valor)
        .expect(MENSAGEM_DE_FALHA);

    println!("{}", "Digite o segundo valor a ser somado".cyan());
    let mut segundo_valor = String::new();

    io::stdin()
        .read_line(&mut segundo_valor)
        .expect(MENSAGEM_DE_FALHA);

    let primeiro_valor: i32 = primeiro_valor.trim().parse()
        .expect(MENSAGEM_DE_FALHA_INTEIRO);
    let segundo_valor: i32 = segundo_valor.trim().parse()
        .expect(MENSAGEM_DE_FALHA_INTEIRO);

    (primeiro_valor, segundo_valor)
}