#[derive(Debug)]
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
    constants: Vec<Constant>,
}

impl ConstantPool {
    pub fn add(&mut self, constant: Constant) {
        self.constants.push(constant);
    }
}
