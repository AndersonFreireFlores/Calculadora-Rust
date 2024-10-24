use std::io;

fn main() {
    loop {
        println!("Calculadora");
        println!("Insira um digito:");

        let mut x: String = String::new();

        io::stdin().read_line(&mut x)
            .expect("Failed to read line");

        println!("Insira uma operacão:");

        let mut op: String = String::new();

        io::stdin().read_line(&mut op)
            .expect("Failed to read line");

        println!("Insira outro digito:");

        let mut y: String = String::new();

        io::stdin().read_line(&mut y)
            .expect("Failed to read line");

        let x: u32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let y: u32 = match y.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let operacao = op.trim(); // Trim the input

        match operacao {
            "+" => println!("{}", x + y),
            "-" => println!("{}", x - y),
            "*" => println!("{}", x * y),
            "x" => println!("{}", x * y),
            "X" => println!("{}", x * y),
            "/" => println!("{}", x / y),
            "%" => println!("{}", x % y),
            _ => {
                println!("Não é uma operacão valida!");
                continue;
            },
        }
        break;
    }
}