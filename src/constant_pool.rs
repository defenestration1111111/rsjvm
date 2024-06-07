use name_variant::NamedVariant;

#[derive(Debug, thiserror::Error)]
pub enum ConstantPoolError {
    #[error("Index out of bounds at index {0}")]
    #[non_exhaustive]
    IndexOutOfBounds(usize),
    #[error("Accessing unusable constant at index {0}")]
    #[non_exhaustive]
    UnsuableConstant(usize),
}

#[derive(Debug, Clone, PartialEq, NamedVariant)]
pub enum Constant {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    ClassIndex(u16),
    StringIndex(u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    NameAndType(u16, u16),
    MethodHandle(u8, u16),
    MethodType(u16),
    Dynamic(u16, u16),
    InvokeDynamic(u16, u16),
    Module(u16),
    Package(u16),
    Unsuable,
}

impl Constant {
    pub fn name(self) -> String {
        self.variant_name().to_string()
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConstantPool {
    constants: Vec<Constant>,
}

impl ConstantPool {
    pub fn add(&mut self, constant: Constant) {
        self.constants.push(constant.clone());
        if matches!(constant, Constant::Long(_) | Constant::Double(_)) {
            self.constants.push(Constant::Unsuable);
        }
    }

    pub fn get(&self, index: usize) -> Result<&Constant, ConstantPoolError> {
        match self.constants.get(index) {
            Some(constant) if matches!(constant, Constant::Unsuable) => Err(ConstantPoolError::UnsuableConstant(index)),
            Some(constant) => Ok(constant),
            None => Err(ConstantPoolError::IndexOutOfBounds(index)),
        }
    }
}
