use std::io;

struct Calculator <T, U> {
    x: T,
    y: U
}

impl<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy, U: std::ops::Div<Output = T> + Copy> Calculator<T, U> {
    fn new(x: T, y: U) -> Self {
        Calculator { x, y }
    }

    fn add(&self) -> T {
        self.x + self.y
    }

    fn sub(&self) -> T {
        self.x - self.y
    }

    fn mul(&self) -> T {
        self.x * self.y
    }

    fn div(&self) -> Result<T, &'static str> {
        if self.y != 0.0 {
            Ok(self.x / self.y)
        } else {
            Err("на нуль ділити не можна")
        }
    }
}

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut operation = String::new();

    println!("Введіть x: ");
    match io::stdin().read_line(&mut x) {
        Ok(_) => {},
        Err(err) => println!("Помилка - {}", err)
    }

    println!("Введіть y: ");
    match io::stdin().read_line(&mut y) {
        Ok(_) => {},
        Err(err) => println!("Помилка - {}", err)
    }

    println!("Введіть дію (+, -, /, *): ");
    match io::stdin().read_line( &mut operation) {
        Ok(_) => {},
        Err(err) => println!("Помилка - {}", err)
    }

    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Неправильний формат x. Використовуйте числа.");
            return;
        }
    };

    let y: f64 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Неправильний формат y. Використовуйте числа.");
            return;
        }
    };

    let calculator = Calculator::new(x, y);

    println!("Введіть дію (+, -, /, *): ");
    match io::stdin().read_line(&mut operation) {
        Ok(_) => {}
        Err(err) => println!("Помилка - {}", err),
    }

    let operation = operation.trim();

    let result = match operation {
        "+" => calculator.add(),
        "-" => calculator.sub(),
        "*" => calculator.mul(),
        "/" => calculator.div(),
        _ => {
            println!("Невідома операція");
            return;
        }
    };

    match result {
        Ok(res) => println!("Результат: {}", res),
        Err(err) => println!("{}", err),
    }
}
