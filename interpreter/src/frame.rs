#![allow(dead_code)]

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    // Ref(*mut u8),
    Null,
    /* ReturnAddress(usize), */
}

#[derive(Debug)]
struct OperandStack {
    elements: Vec<Value>,
    capacity: usize,
}

impl OperandStack {
    pub fn new(capacity: usize) -> Self {
        Self { elements: Vec::with_capacity(capacity), capacity }
    }

    pub fn push(&mut self, value: Value) {
        self.elements.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.elements.pop().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

pub struct Frame {
    locals: Vec<Value>,
    stack: OperandStack,
    pc: usize,
}

impl Frame {
    pub fn new(max_locals: usize, max_stack: usize) -> Self {
        Self {
            locals: Vec::with_capacity(max_locals),
            stack: OperandStack::new(max_stack),
            pc: 0,
        }
    }

    pub fn push_operand(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop_operand(&mut self) -> Value {
        self.stack.pop()
    }
}

pub trait Operand: Copy {
    fn pop(frame: &mut Frame) -> Self;
    fn push(frame: &mut Frame, value: Self);
}

impl Operand for i32 {
    fn pop(frame: &mut Frame) -> Self {
        match frame.pop_operand() {
            Value::Int(value) => value,
            _ => panic!("Expected i32"),
        }
    }

    fn push(frame: &mut Frame, value: Self) {
        frame.push_operand(Value::Int(value));
    }
}

impl Operand for i64 {
    fn pop(frame: &mut Frame) -> Self {
        match frame.pop_operand() {
            Value::Long(value) => value,
            _ => panic!("Expected i64"),
        }
    }

    fn push(frame: &mut Frame, value: Self) {
        frame.push_operand(Value::Long(value));
    }
}

impl Operand for f32 {
    fn pop(frame: &mut Frame) -> Self {
        match frame.pop_operand() {
            Value::Float(value) => value,
            _ => panic!("Expected f32"),
        }
    }

    fn push(frame: &mut Frame, value: Self) {
        frame.push_operand(Value::Float(value));
    }
}

impl Operand for f64 {
    fn pop(frame: &mut Frame) -> Self {
        match frame.pop_operand() {
            Value::Double(value) => value,
            _ => panic!("Expected f64"),
        }
    }

    fn push(frame: &mut Frame, value: Self) {
        frame.push_operand(Value::Double(value));
    }
}

pub fn binary_op<T: Operand, F: Fn(T, T) -> T>(op: F, frame: &mut Frame) {
    let second = T::pop(frame);
    let first = T::pop(frame);
    let result = op(first, second);
    T::push(frame, result);
}

pub fn binary_op_return<T: Operand, R: Operand>(op: impl Fn(T, T) -> R, frame: &mut Frame) {
    let second = T::pop(frame);
    let first = T::pop(frame);
    let result = op(first, second);
    R::push(frame, result);
}

pub fn unary_op<T: Operand, F: Fn(T) -> T>(op: F, frame: &mut Frame) {
    let value = T::pop(frame);
    let result = op(value);
    T::push(frame, result);
}
