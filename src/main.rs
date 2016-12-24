use std::io;

struct Calculator {
    n1: f64,
    n2: f64
}

impl Calculator {
    fn new(n1: f64, n2: f64) -> Calculator {
        Calculator {
            n1: n1,
            n2: n2
        }
    }

    fn add(&self) -> f64 {
        self.n1 + self.n2
    }

    fn multi(&self) -> f64 {
        self.n1 * self.n2
    }

    fn divide(&self) -> f64 {
        self.n1 / self.n2
    }

    fn rest_divide(&self) -> f64 {
        self.n1 % self.n2
    }
}

fn main() {
    // Initalize with some values
    let mut c = Calculator::new(0.0, 0.0);

    create_calculator(&mut c);
    let r: f64 = select_operation(c);
    println!("Your result is: {}", r);
}

fn create_calculator(calc: &mut Calculator) {
    let mut f1 = String::new();
    println!("Type in your first number");
    io::stdin().read_line(&mut f1).expect("Unable to read input");
    let f1: f64 = match f1.trim().parse() {
        Ok(v) => v,
        Err(r) => panic!("Error with reason: {}", r)
    };

    let mut f2 = String::new();
    println!("Type in your second number");
    io::stdin()
        .read_line(&mut f2)
        .expect("Unable to read input");

    let f2: f64 = match f2.trim().parse() {
        Ok(v) => v,
        Err(r) => panic!("Error with reason: {}", r)
    };

    *calc = Calculator::new(f1, f2);
}

fn select_operation(calc: Calculator) -> f64 {
    println!(
        "Select one of these operations\n
        1 > Addition\n
        2 > Multiplication\n
        3 > Division\n"
    );

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("An error occured while choosing selection");

    let choice: i8 = match choice.trim().parse() {
        Ok(v) => v,
        Err(r) => panic!("Error w/ reason: {}", r)
    };

    if choice == 1 {
        calc.n1 + calc.n2
    } else if choice == 2 {
        calc.n1 * calc.n2
    } else if choice == 3 {
        calc.n1 / calc.n2
    } else {
        panic!("Something went wrong")
    }
}
