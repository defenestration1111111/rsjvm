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
        Self {
            elements: Vec::with_capacity(capacity),
            capacity,
        }
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