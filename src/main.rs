use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() { 
    println!("Adivinhe o número!");

    let numero = rand::thread_rng().gen_range(1..=100);
    let mut tentativas = 0;
    
    loop {
        println!("Deixe seu palpite: ");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Erro ao ler palpite!");

        match palpite.trim().parse::<u32>() {
            Err(_) => {
                println!("Input inválido!");
                continue;
            }
            Ok(palpite) => {
                tentativas += 1;
                match palpite.cmp(&numero) {
                    Ordering::Less => println!("Seu palpite é menor que o número secreto!"),
                    Ordering::Greater => println!("Seu palpite é maior que o número secreto!"),
                    Ordering::Equal => {
                        println!("Você venceu com {tentativas} tentativas!");
                        break;
                    },
                }
            }
        }
    }
    
}
