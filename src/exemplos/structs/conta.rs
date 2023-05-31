struct Conta {
    titular: Titular,
    saldo: f64,
}

struct Titular {
    nome: String,
    sobrenome: String,
    cpf: String,
    idade: u8,
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        if self.saldo >= valor {
            self.saldo -= valor;
        } else {
            println!("Saldo insuficiente!");
        }
    }
}

fn main() {
    let mut conta = Conta {
        titular: Titular {
            nome: String::from("Elizabeth"),
            sobrenome: String::from("Espinoza"),
            cpf: String::from("111.111.111-11"),
            idade: 100,
        },
        saldo: 100.0,
    };

    println!(
        "Titular: {} {}, CPF: {}, Idade: {}",
        conta.titular.nome, conta.titular.sobrenome, conta.titular.cpf, conta.titular.idade,
    );

    println!("Saldo: {}", conta.saldo);

    println!("Sacando 50.5");

    conta.sacar(50.5);

    println!("Saldo: {}", conta.saldo);
}
