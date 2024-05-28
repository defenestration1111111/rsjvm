use crate::constant_pool::Constant;

#[derive(Debug)]
pub struct ConstantValue {
    value: Constant,
}

impl ConstantValue {
    pub fn new(constant_value: Constant) -> Self {
        ConstantValue { value: constant_value }
    }
}
