use std::io;

use csv::WriterBuilder;
use serde::Serialize;

enum Function {
    X2,
    X3,
    Sin,
    Cos,
    Exp,
}

impl Function {
    fn calculate(&self, x: f64) -> f64 {
        match self {
            Function::X2 => x * x,
            Function::X3 => x * x * x,
            Function::Sin => x.sin(),
            Function::Cos => x.cos(),
            Function::Exp => x.exp()
        }
    }

    fn derivative(&self, x: f64) -> f64 {
        match self {
            Function::X2 => 2 * x,
            Function::X3 => 3 * x,
            Function::Sin => x.cos(),
            Function::Cos => -x.sin(),
            Function::Exp => x.exp()
        }
    }
}


fn main() {
    let mut input = String::new();
    println!("Выберите функцию:\n1. x^2\n2. x^3\n3. sin\n4. cos\n5. e^x\n");
    io::stdin().read_line(&mut input).unwrap();

    let func_type: u8 = input.trim().parse().unwrap();
    let fx =  match func_type {
        1 => Function::X2,
        2 => Function::X3,
        3 => Function::Sin,
        4 => Function::Cos,
        5 => Function::Exp,
        _ => panic!("Unexpected ind")
    };

    input = String::new();
    println!("Введите x: ");
    io::stdin().read_line(&mut input).unwrap();

    let x = input.trim().parse::<f64>().unwrap();

    let mut index: u8 = 1;
    for h in [0.1, 0.01, 0.001] {
        let right_difference = (fx.calculate(x+h) - fx.calculate(x)) / h;

    }
}