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
}

fn main() {
    let mut f1 = String::new();
    io::stdin().read_line(&mut f1).expect("Unable to read input");
    let f1: f64 = match f1.trim().parse() {
        Ok(v) => v,
        Err(r) => panic!("Error with reason: {}", r)
    };

    let mut f2 = String::new();
    io::stdin().read_line(&mut f2).expect("Unable to read input");
    let f2: f64 = match f2.trim().parse() {
        Ok(v) => v,
        Err(r) => panic!("Error with reason: {}", r)
    };


    let mut c = Calculator::new(f1, f2);
    println!("{:?}", c.add());
}
