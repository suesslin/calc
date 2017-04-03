use operations::{Op, choose_op};

pub struct Calc {
    first_num: Option<usize>,
    op: Option<Op>,
    second_num: Option<usize>,
}

impl Calc {
    pub fn new() -> Calc {
        Calc {
            first_num: None,
            op: None,
            second_num: None
        }
    }
}

// Setter for Calculator struct

pub trait CalcSetter {
    fn set_first(&mut self, num: usize);
    fn set_second(&mut self, num: usize);
    fn set_op(&mut self, op: Op);
}

impl CalcSetter for Calc {
    fn set_first(&mut self, num: usize) {
        self.first_num = Some(num)
    }

    fn set_second(&mut self, num: usize) {
        self.second_num = Some(num)
    }

    fn set_op(&mut self, op: Op) {
        self.op = Some(op)
    }
}

// Getter for Calculator struct

pub trait CalcGetter {
    fn get_result(self) -> usize;
}

impl CalcGetter for Calc {
    fn get_result(self) -> usize {
        match (self.first_num, self.second_num) {
            // NOTE: op unwrapping is not a clever idea!
            (Some(x), Some(y)) => choose_op(&self.op.unwrap(), (&x, &y)),
            (None, None) => panic!("No number set."),
            (None, Some(_)) => panic!("First number not set."),
            (Some(_), None) => panic!("Second number not set.")
        }
    }
}
