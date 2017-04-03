pub fn with_op(op: fn (&usize, &usize) -> usize, nums: (&usize, &usize)) -> usize {
    op(nums.0, nums.1)
}

// Basic Arithmetic operations

pub fn add(num1: &usize, num2: &usize) -> usize {
    num1 + num2
}

pub fn subtract(num1: &usize, num2: &usize) -> usize {
    num1 - num2
}

pub fn multiply(num1: &usize, num2: &usize) -> usize {
    num1 * num2
}

pub fn divide(num1: &usize, num2: &usize) -> usize {
    num1 / num2
}

pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub fn choose_op(op: &Op, nums: (&usize, &usize)) -> usize {
    match op {
        &Op::Add => with_op(add, nums),
        &Op::Subtract => with_op(subtract, nums),
        &Op::Multiply => with_op(multiply, nums),
        &Op::Divide => with_op(divide, nums)
    }
}
