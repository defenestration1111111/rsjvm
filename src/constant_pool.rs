#[derive(Debug, Clone)]
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
}

#[derive(Debug, Default)]
pub struct ConstantPool {
    constants: Vec<Option<Constant>>,
}

impl ConstantPool {
    pub fn add(&mut self, index: u16, constant: Constant) {
        self.constants[index as usize] = Some(constant);
    }

    pub fn get(&self, index: usize) -> Option<&Constant> {
        self.constants.get(index).and_then(|opt| opt.as_ref())
    }
}
