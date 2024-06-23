use std::fmt;

use crate::access_flag::ClassFileAccessFlags;
use crate::attribute::Attribute;
use crate::attribute::UserDefinedAttribute;
use crate::byte_reader::ByteReader;
use crate::byte_reader::ReadError;
use crate::class_file::ClassFile;
use crate::class_file_version::{ClassFileVersion, FileVersionError};
use crate::constant_pool::Constant;
use crate::constant_pool::ConstantPoolError;
use crate::field::BaseType;
use crate::field::Field;
use crate::field::FieldAccessFlags;
use crate::field::FieldError;
use crate::field::FieldType;
use crate::instruction::Instruction;
use crate::method::Method;
use crate::method::MethodAccessFlags;
use crate::method::MethodDescriptor;
use crate::method::MethodParsingError;
use crate::predefined_attributes::Code;
use crate::predefined_attributes::ConstantValue;
use crate::predefined_attributes::SourceFile;
use crate::predefined_attributes::StackMapFrame;
use crate::predefined_attributes::StackMapTable;
use crate::predefined_attributes::VerificationTypeInfo;

type Result<T> = std::result::Result<T, ClassReaderError>;

#[derive(Debug, thiserror::Error)]
pub enum ClassReaderError {
    #[error("Invalid magic number {0}")]
    #[non_exhaustive]
    InvalidMagicNumber(u32),
    #[error("Tag not supported: {0}")]
    #[non_exhaustive]
    TagNotSupported(u8),
    #[error("Unexpected constant: expected {expected:?}, found {actual:?}")]
    #[non_exhaustive]
    UnexpectedConstant {
        expected: String,
        actual: String
    },
    #[error("Mismatched constant type for field type")]
    #[non_exhaustive]
    MismatchedConstantType(FieldType, u16),
    #[error("Invalid attribute data size {0}. Expected {1}")]
    #[non_exhaustive]
    InvalidAttributeSize(u32, u32),
    #[error("Invalid verification type {0}")]
    #[non_exhaustive]
    InvalidVerificationType(u8),
    #[error("Frame type {0} is not supported")]
    #[non_exhaustive]
    InvalidStackMapFrameType(u8),
    #[error("Attribute name index of the SourceFile attribute must represent the string 'SourceFile', actual: {0}")]
    #[non_exhaustive]
    InvalidSourceFileString(String),
    #[error("Error encountered during reading: {0}")]
    #[non_exhaustive]
    ReadError(#[from] ReadError),
    #[error("Error during parsing file version: {0}")]
    #[non_exhaustive]
    FileVersionError(#[from] FileVersionError),
    #[error("Error during parsing constant pool {0}")]
    #[non_exhaustive]
    ConstantPoolError(#[from] ConstantPoolError),
    #[error("Error while parsing field: {0}")]
    #[non_exhaustive]
    FieldError(#[from] FieldError),
    #[error("Error while parsing method: {0}")]
    #[non_exhaustive]
    MethodParsingError(#[from] MethodParsingError),
}

pub struct ContextualError {
    err: ClassReaderError,
    snippet: Vec<u8>,
}

impl ContextualError {
    fn new(err: ClassReaderError, snippet: Vec<u8>) -> Self {
        ContextualError { err, snippet }
    }
}

impl fmt::Debug for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContextualError {{ err: {:?}, snippet: [", self.err)?;
        for (i, byte) in self.snippet.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "0x{:02X}", byte)?;
        }
        write!(f, "] }}")
    }
}

#[derive(Debug, Clone)]
pub struct ClassFileReader<'a> {
    byte_reader: ByteReader<'a>,
    class_file: ClassFile,
}

impl<'a> ClassFileReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        ClassFileReader {
            byte_reader: ByteReader::new(data),
            class_file: ClassFile::default(),
        }
    }

    pub fn read_class(data: &[u8]) -> std::result::Result<ClassFile, ContextualError> {
        let mut class_reader = ClassFileReader::new(data);
        let result = class_reader.read();
        match result {
            Ok(class_file) => Ok(class_file),
            Err(err) => {
                let snippet = class_reader.snippet();
                Err(ContextualError::new(err, snippet))
            }
        }
    }

    fn snippet(&self) -> Vec<u8> {
        self.byte_reader.snippet()
    }

    fn read(&mut self) -> Result<ClassFile> {
        self.read_magic_number()?;
        self.read_version()?;
        self.read_constant_pool()?;
        self.read_access_flags()?;
        self.read_this_class()?;
        self.read_super_class()?;
        self.read_interfaces()?;
        self.read_fields()?;
        self.read_methods()?;
        // self.read_class_attributes()?;
        Ok(self.class_file.clone())
    }

    pub fn read_magic_number(&mut self) -> Result<()> {
        match self.byte_reader.read_u32() {
            Ok(0xCAFEBABE) => Ok(()),
            Ok(magic_number) => Err(ClassReaderError::InvalidMagicNumber(magic_number)),
            Err(err) => Err(err.into()),
        }
    }

    pub fn read_version(&mut self) -> Result<()> {
        let minor_version = self.byte_reader.read_u16()?;
        let major_version = self.byte_reader.read_u16()?;
        self.class_file.version = ClassFileVersion::from(major_version, minor_version)?;
        Ok(())
    }

    pub fn read_constant_pool(&mut self) -> Result<()> {
        let constant_pool_count = self.byte_reader.read_u16()?;
        for mut index in 1..constant_pool_count {
            let tag = self.byte_reader.read_u8()?;
            let constant = match tag {
                1 => self.read_string_constant()?,
                3 => self.read_int_constant()?,
                4 => self.read_float_constant()?,
                5 => self.read_long_constant()?,
                6 => self.read_double_constant()?,
                7 => self.read_class_index()?,
                8 => self.read_string_info()?,
                9 => self.read_field_ref()?,
                10 => self.read_method_ref()?,
                11 => self.read_interface_method_ref()?,
                12 => self.read_name_and_type()?,
                15 => self.read_method_handle()?,
                16 => self.read_method_type()?,
                17 => self.read_dynamic()?,
                18 => self.read_invoke_dynamic()?,
                19 => self.read_module()?,
                20 => self.read_package()?,
                _ => return Err(ClassReaderError::TagNotSupported(tag)),
            };

            self.class_file.constant_pool.add(constant.clone());

            if matches!(constant, Constant::Long(_) | Constant::Double(_)) {
                index += 1;
            }
        }
        Ok(())
    }

    fn read_string_constant(&mut self) -> Result<Constant> {
        let length = self.byte_reader.read_u16()?;
        Ok(Constant::Utf8(self.byte_reader.read_utf8(length as u32)?.into_owned()))
    }

    fn read_int_constant(&mut self) -> Result<Constant> {
        let integer = self.byte_reader.read_i32()?;
        Ok(Constant::Integer(integer))
    }

    fn read_float_constant(&mut self) -> Result<Constant> {
        let float = self.byte_reader.read_f32()?;
        Ok(Constant::Float(float))
    }

    fn read_long_constant(&mut self) -> Result<Constant> {
        let long = self.byte_reader.read_i64()?;
        Ok(Constant::Long(long))
    }

    fn read_double_constant(&mut self) -> Result<Constant> {
        let double = self.byte_reader.read_f64()?;
        Ok(Constant::Double(double))
    }

    fn read_class_index(&mut self) -> Result<Constant> {
        let class_index = self.byte_reader.read_u16()?;
        Ok(Constant::ClassIndex(class_index))
    }

    fn read_string_info(&mut self) -> Result<Constant> {
        let string_info = self.byte_reader.read_u16()?;
        Ok(Constant::StringIndex(string_info))
    }

    fn read_field_ref(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::FieldRef(pair.0, pair.1))
    }

    fn read_method_ref(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::MethodRef(pair.0, pair.1))
    }

    fn read_interface_method_ref(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::InterfaceMethodRef(pair.0, pair.1))
    }

    fn read_name_and_type(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::NameAndType(pair.0, pair.1))
    }

    fn read_method_handle(&mut self) -> Result<Constant> {
        let reference_kind = self.byte_reader.read_u8()?;
        let reference_index = self.byte_reader.read_u16()?;
        Ok(Constant::MethodHandle(reference_kind, reference_index))
    }

    fn read_method_type(&mut self) -> Result<Constant> {
        let descriptor_index = self.byte_reader.read_u16()?;
        Ok(Constant::MethodType(descriptor_index))
    }

    fn read_dynamic(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::Dynamic(pair.0, pair.1))
    }

    fn read_invoke_dynamic(&mut self) -> Result<Constant> {
        let pair = self.byte_reader.read_pair_u16()?;
        Ok(Constant::InvokeDynamic(pair.0, pair.1))
    }

    fn read_module(&mut self) -> Result<Constant> {
        let name_index = self.byte_reader.read_u16()?;
        Ok(Constant::Module(name_index))
    }

    fn read_package(&mut self) -> Result<Constant> {
        let name_index = self.byte_reader.read_u16()?;
        Ok(Constant::Package(name_index))
    }

    fn read_access_flags(&mut self) -> Result<()> {
        let mask = self.byte_reader.read_u16()?;
        let flags = ClassFileAccessFlags::new(mask);
        self.class_file.flags = flags;
        Ok(())
    }

    fn get_utf8(&mut self, name_index: u16) -> Result<String> {
        let constant = self.class_file.constant_pool.get(name_index as usize)?;
        match constant {
            Constant::Utf8(utf8_content) => Ok(utf8_content.clone()),
            _ => Err(ClassReaderError::UnexpectedConstant { expected: "Utf8".to_string(), actual: constant.name() }),
        }
    }

    fn check_utf8(&mut self, name_index: u16, string: &str) -> Result<()> {
        let utf8 = self.get_utf8(name_index)?;
        if utf8 == string {
            Ok(())
        } else {
            Err(ClassReaderError::InvalidAttributeNameIndex(string.to_string(), utf8))
        }
    }

    fn get_class_name(&mut self, name_index: u16) -> Result<String> {
        let constant = self.class_file.constant_pool.get(name_index as usize)?;
        match constant {
            Constant::ClassIndex(class_index) => self.get_utf8(*class_index),
            _ => Err(ClassReaderError::UnexpectedConstant { expected: "ClassIndex".to_string(), actual: constant.name() })
        }
    }

    fn read_this_class(&mut self) -> Result<()> {
        let name_index = self.byte_reader.read_u16()?;
        self.class_file.this_class = self.get_class_name(name_index)?;
        Ok(())
    }

    fn read_super_class(&mut self) -> Result<()> {
        let name_index = self.byte_reader.read_u16()?;
        if name_index == 0 {
            return Ok(())
        }
        self.class_file.super_class = Some(self.get_class_name(name_index)?);
        Ok(())
    }

    fn read_interfaces(&mut self) -> Result<()> {
        let interfaces_count = self.byte_reader.read_u16()?;
        let mut intefaces = Vec::new();
        for _ in 0..interfaces_count {
            let name_index = self.byte_reader.read_u16()?;
            intefaces.push(self.get_class_name(name_index)?);
        }
        self.class_file.interfaces = intefaces;
        Ok(())
    }

    fn read_fields(&mut self) -> Result<()> {
        let fields_count = self.byte_reader.read_u16()?;
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(self.read_field()?);
        }
        self.class_file.fields = fields;
        Ok(())
    }

    fn read_field(&mut self) -> Result<Field> {
        let (access_flags, name_index) = self.byte_reader.read_pair_u16()?;
        let flags = FieldAccessFlags::new(access_flags);
        let name = self.get_utf8(name_index)?;

        let (descriptor_index, attributes_count) = self.byte_reader.read_pair_u16()?;
        let field_descriptor_utf8 = self.get_utf8(descriptor_index)?;
        let type_descriptor = FieldType::try_from(
                &mut field_descriptor_utf8
                .chars()
                .peekable()
        )?;

        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            let name_index = self.byte_reader.read_u16()?;
            let name = self.get_utf8(name_index)?;
            let attr = match name.as_str() {
                "ConstantValue" => self.read_constant_value_attr(type_descriptor.clone())?,
                _ => self.read_user_defined_attr(name)?,
            };
            attributes.push(attr);
        }
        Ok(Field::new(flags, name, type_descriptor, attributes))
    }

    fn read_methods(&mut self) -> Result<()> {
        let methods_count = self.byte_reader.read_u16()?;
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(self.read_method()?);
        }
        self.class_file.methods = methods;
        Ok(())
    }

    fn read_method(&mut self) -> Result<Method> {
        let (access_flag, name_index) = self.byte_reader.read_pair_u16()?;
        let flags = MethodAccessFlags::new(access_flag);
        let name = self.get_utf8(name_index)?;

        let (descriptor_index, attributes_count) = self.byte_reader.read_pair_u16()?;
        let type_descriptor = MethodDescriptor::try_from(
            &mut self.get_utf8(descriptor_index)?
                .chars()
                .peekable() 
        )?;

        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            let name_index = self.byte_reader.read_u16()?;
            let name = self.get_utf8(name_index)?;
            let attr = match name.as_str() {
                "Code" => self.read_code_attr()?,
                _ => self.read_user_defined_attr(name)?,
            };
            attributes.push(attr);
        }
        Ok(Method { flags, name, type_descriptor, attributes })
    }

    fn read_constant_value_attr(&mut self, field_type: FieldType) -> Result<Attribute> {
        let length = self.byte_reader.read_u32()?;
        if length != 2 {
            return Err(ClassReaderError::InvalidAttributeSize(length, 2))
        }
        let constantvalue_index = self.byte_reader.read_u16()?;
        let constant_value = self.class_file.constant_pool.get(constantvalue_index as usize)?;
        match (field_type.clone(), constant_value) {
            (FieldType::Base(BaseType::Int), Constant::Integer(_)) |
            (FieldType::Base(BaseType::Short), Constant::Integer(_)) |
            (FieldType::Base(BaseType::Char), Constant::Integer(_)) |
            (FieldType::Base(BaseType::Byte), Constant::Integer(_)) |
            (FieldType::Base(BaseType::Boolean), Constant::Integer(_)) => Ok(Attribute::ConstantValue(ConstantValue::new(constant_value.clone()))),
            (FieldType::Base(BaseType::Float), Constant::Float(_)) => Ok(Attribute::ConstantValue(ConstantValue::new(constant_value.clone()))),
            (FieldType::Base(BaseType::Long), Constant::Long(_)) => Ok(Attribute::ConstantValue(ConstantValue::new(constant_value.clone()))),
            (FieldType::Base(BaseType::Double), Constant::Double(_)) => Ok(Attribute::ConstantValue(ConstantValue::new(constant_value.clone()))),
            (FieldType::Object(ref class_name), Constant::Utf8(_)) if class_name == "java/lang/String" => Ok(Attribute::ConstantValue(ConstantValue::new(constant_value.clone()))),
            _ => Err(ClassReaderError::MismatchedConstantType(field_type, constantvalue_index)),
        }
    }

    fn read_code_attr(&mut self) -> Result<Attribute> {
        let length = self.byte_reader.read_u32()?;
        let max_stack = self.byte_reader.read_u16()?;
        let max_locals = self.byte_reader.read_u16()?;
        let code_length = self.byte_reader.read_u32()?;

        let mut instructions = Vec::new();
        let mut bytes_read = 0;

        while bytes_read < code_length {
            let index = self.byte_reader.read_u8()?;
            instructions.push(self.read_instruction(index, &mut bytes_read)?);
        }
        Ok(Attribute::Code(Code { max_stack, max_locals, code: instructions, exception_table: Vec::new(), attributes: Vec::new() }))
    }

    fn read_instruction(&mut self, index: u8, address: &mut u32) -> Result<(Instruction, u32)> {
        let current_address: u32 = *address;
        *address += 1;

        let instruction = match index {
            0x32 => Instruction::Aaload,
            0x53 => Instruction::Aastore,
            0x01 => Instruction::Aconst_null,
            0x19 => Instruction::Aload(self.read_instruction_u8(address)?),
            0x2a => Instruction::Aload_0,
            0x2b => Instruction::Aload_1,
            0x2c => Instruction::Aload_2,
            0x2d => Instruction::Aload_3,
            0xbd => Instruction::Anewarray(self.read_instruction_u16(address)?),
            0xb0 => Instruction::Areturn,
            0xbe => Instruction::Arraylength,
            0x3a => Instruction::Astore(self.read_instruction_u8(address)?),
            0x4b => Instruction::Astore_0,
            0x4c => Instruction::Astore_1,
            0x4d => Instruction::Astore_2,
            0x4e => Instruction::Astore_3,
            0xbf => Instruction::Athrow,
            0x33 => Instruction::Baload,
            0x54 => Instruction::Bastore,
            0x10 => Instruction::Bipush(self.read_instruction_u8(address)?),
            0x34 => Instruction::Caload,
            0x55 => Instruction::Castore,
            0xc0 => Instruction::Checkcast(self.read_instruction_u16(address)?),
            0x90 => Instruction::D2f,
            0x8e => Instruction::D2i,
            0x8f => Instruction::D2l,
            0x63 => Instruction::Dadd,
            0x31 => Instruction::Daload,
            0x52 => Instruction::Dastore,
            0x98 => Instruction::Dcmpg,
            0x97 => Instruction::Dcmpl,
            0x0e => Instruction::Dconst_0,
            0x0f => Instruction::Dconst_1,
            0x6f => Instruction::Ddiv,
            0x18 => Instruction::Dload(self.read_instruction_u8(address)?),
            0x26 => Instruction::Dload_0,
            0x27 => Instruction::Dload_1,
            0x28 => Instruction::Dload_2,
            0x29 => Instruction::Dload_3,
            0x6b => Instruction::Dmul,
            0x77 => Instruction::Dneg,
            0x73 => Instruction::Drem,
            0xaf => Instruction::Dreturn,
            0x39 => Instruction::Dstore(self.read_instruction_u8(address)?),
            0x47 => Instruction::Dstore_0,
            0x48 => Instruction::Dstore_1,
            0x49 => Instruction::Dstore_2,
            0x4a => Instruction::Dstore_3,
            0x67 => Instruction::Dsub,
            0x59 => Instruction::Dup,
            0x5a => Instruction::Dup_x1,
            0x5b => Instruction::Dup_x2,
            0x5c => Instruction::Dup_2,
            0x5d => Instruction::Dup2_x1,
            0x5e => Instruction::Dup2_x2,
            0x8d => Instruction::F2d,
            0x8b => Instruction::F2i,
            0x8c => Instruction::F2l,
            0x62 => Instruction::Fadd,
            0x30 => Instruction::Faload,
            0x51 => Instruction::Fastore,
            0x96 => Instruction::Fcmpg,
            0x95 => Instruction::Fcmpl,
            0x0b => Instruction::Fconst_0,
            0x0c => Instruction::Fconst_1,
            0x0d => Instruction::Fconst_2,
            0x6e => Instruction::Fdiv,
            0x17 => Instruction::Fload(self.read_instruction_u8(address)?),
            0x22 => Instruction::Fload_0,
            0x23 => Instruction::Fload_1,
            0x24 => Instruction::Fload_2,
            0x25 => Instruction::Fload_3,
            0x6a => Instruction::Fmul,
            0x76 => Instruction::Fneg,
            0x72 => Instruction::Frem,
            0xae => Instruction::Freturn,
            0x38 => Instruction::Fstore(self.read_instruction_u8(address)?),
            0x43 => Instruction::Fstore_0,
            0x44 => Instruction::Fstore_1,
            0x45 => Instruction::Fstore_2,
            0x46 => Instruction::Fstore_3,
            0x66 => Instruction::Fsub,
            0xb4 => Instruction::Getfield(self.read_instruction_u16(address)?),
            0xb2 => Instruction::Getstatic(self.read_instruction_u16(address)?),
            0xa7 => todo!("Goto"),
            0xc8 => todo!("Goto_w"),
            0x91 => Instruction::I2b,
            0x92 => Instruction::I2c,
            0x87 => Instruction::I2d,
            0x86 => Instruction::I2f,
            0x85 => Instruction::I2l,
            0x93 => Instruction::I2s,
            0x60 => Instruction::Iadd,
            0x2e => Instruction::Iaload,
            0x7e => Instruction::Iand,
            0x4f => Instruction::Iastore,
            0x02 => Instruction::Iconst_m1,
            0x03 => Instruction::Iconst_0,
            0x04 => Instruction::Iconst_1,
            0x05 => Instruction::Iconst_2,
            0x06 => Instruction::Iconst_3,
            0x07 => Instruction::Iconst_4,
            0x08 => Instruction::Iconst_5,
            0x6c => Instruction::Idiv,
            0xa5 => Instruction::If_acmpeq(self.byte_reader.read_u16()?),
            0xa6 => Instruction::If_acmpne(self.byte_reader.read_u16()?),
            0x9f => Instruction::If_icmpeq(self.byte_reader.read_u16()?),
            0xa0 => Instruction::If_icmpne(self.byte_reader.read_u16()?),
            0xa1 => Instruction::If_icmplt(self.byte_reader.read_u16()?),
            0xa2 => Instruction::If_icmpge(self.byte_reader.read_u16()?),
            0xa3 => Instruction::If_icmpgt(self.byte_reader.read_u16()?),
            0xa4 => Instruction::If_icmple(self.byte_reader.read_u16()?),
            0x99 => Instruction::Ifeq(self.byte_reader.read_u16()?),
            0x9a => Instruction::Ifne(self.byte_reader.read_u16()?),
            0x9b => Instruction::Iflt(self.byte_reader.read_u16()?),
            0x9c => Instruction::Ifge(self.byte_reader.read_u16()?),
            0x9d => Instruction::Ifgt(self.byte_reader.read_u16()?),
            0x9e => Instruction::Ifle(self.byte_reader.read_u16()?),
            0xc7 => Instruction::Ifnonnull(self.byte_reader.read_u16()?),
            0xc6 => Instruction::Ifnull(self.byte_reader.read_u16()?),
            0x84 => Instruction::Iinc(self.read_instruction_u8(address)?, self.read_instruction_i8(address)?),
            0x15 => Instruction::Iload(self.read_instruction_u8(address)?),
            0x1a => Instruction::Iload_0,
            0x1b => Instruction::Iload_1,
            0x1c => Instruction::Iload_2,
            0x1d => Instruction::Iload_3,
            0x68 => Instruction::Imul,
            0x74 => Instruction::Ineg,
            0xc1 => Instruction::Instanceof(self.read_instruction_u16(address)?),
            0xba => Instruction::Invokedynamic(self.read_instruction_u16(address)?),
            0xb7 => Instruction::Invokespecial(self.read_instruction_u16(address)?),
            0xb8 => Instruction::Invokestatic(self.read_instruction_u16(address)?),
            0xb6 => Instruction::Invokevirtual(self.read_instruction_u16(address)?),
            0x80 => Instruction::Ior,
            0x70 => Instruction::Irem,
            0xac => Instruction::Ireturn,
            0x78 => Instruction::Ishl,
            0x7a => Instruction::Ishr,
            0x36 => Instruction::Istore(self.read_instruction_u8(address)?),
            0x3b => Instruction::Istore_0,
            0x3c => Instruction::Istore_1,
            0x3d => Instruction::Istore_2,
            0x3e => Instruction::Istore_3,   
            0x64 => Instruction::Isub,
            0x7c => Instruction::Iushr,
            0x82 => Instruction::Ixor,
            0xa8 => todo!("Jsr"),
            0xc9 => todo!("Jsr_w"),
            0x8a => Instruction::L2d,
            0x89 => Instruction::L2f,
            0x88 => Instruction::L2i,
            0x61 => Instruction::Ladd,
            0x2f => Instruction::Laload,
            0x7f => Instruction::Land,
            0x50 => Instruction::Lastore,
            0x94 => Instruction::Lcmp,
            0x09 => Instruction::Lconst_0,
            0x0a => Instruction::Lconst_1,            
            0x12 => Instruction::Ldc(self.read_instruction_u8(address)?),
            0x13 => Instruction::Ldc_w(self.read_instruction_u16(address)?),
            0x14 => Instruction::Ldc2_w(self.read_instruction_u16(address)?),
            0x6d => Instruction::Ldiv,
            0x16 => Instruction::Lload(self.read_instruction_u8(address)?),
            0x1e => Instruction::Lload_0,
            0x1f => Instruction::Lload_1,
            0x20 => Instruction::Lload_2,
            0x21 => Instruction::Lload_3,
            0x69 => Instruction::Lmul,
            0x75 => Instruction::Lneg,
            0xab => todo!("Lookupswitch"),
            0x81 => Instruction::Lor,
            0x71 => Instruction::Lrem,
            0xad => Instruction::Lreturn,
            0x79 => Instruction::Lshl,
            0x7b => Instruction::Lshr,
            0x37 => Instruction::Lstore(self.read_instruction_u8(address)?),
            0x3f => Instruction::Lstore_0,
            0x40 => Instruction::Lstore_1,
            0x41 => Instruction::Lstore_2,
            0x42 => Instruction::Lstore_3,
            0x65 => Instruction::Lsub,
            0x7d => Instruction::Lushr,
            0x83 => Instruction::Lxor,
            0xc2 => Instruction::Monitorenter,
            0xc3 => Instruction::Monitorexit,
            0xc5 => Instruction::Multianewarray(
                self.read_instruction_u16(address)?, self.read_instruction_u8(address)?
            ),
            0xbb => Instruction::New(self.read_instruction_u16(address)?),
            0xbc => todo!("Newarray"),
            0x00 => Instruction::Nop,
            0x57 => Instruction::Pop,
            0x58 => Instruction::Pop2,
            0xb5 => Instruction::Putfield(self.read_instruction_u16(address)?),
            0xb3 => Instruction::Putstatic(self.read_instruction_u16(address)?),
            0xa9 => Instruction::Ret(self.read_instruction_u8(address)?),
            0xb1 => Instruction::Return,
            0x35 => Instruction::Saload,
            0x56 => Instruction::Sastore,
            0x11 => Instruction::Sipush(self.read_instruction_i16(address)?),
            0x5f => Instruction::Swap,
            0xaa => todo!("Tableswitch"),
            0xc4 => todo!("Wide"),
            _ => panic!("at the disco"), // refactor
        };
        Ok((instruction, current_address))
    }

    fn read_instruction_u8(&mut self, address: &mut u32) -> Result<u8> {
        *address += 1;
        self.byte_reader.read_u8().map_err(|e| e.into())
    }

    fn read_instruction_u16(&mut self, address: &mut u32) -> Result<u16>{
        let index_byte1 = self.read_instruction_u8(address)? as u16;
        let index_byte2 = self.read_instruction_u8(address)? as u16;
        Ok((index_byte1 << 8) | index_byte2)
    }

    fn read_instruction_i8(&mut self, address: &mut u32) -> Result<i8> {
        let byte = self.read_instruction_u8(address)?;
        Ok(byte as i8)
    }

    fn read_instruction_i16(&mut self, address: &mut u32) -> Result<i16> {
        let value = self.read_instruction_u16(address)?;
        Ok(value as i16)
    }

    fn read_stack_map_table_attr(&mut self) -> Result<Attribute> {
        let length = self.byte_reader.read_u32()?;
        let number_of_entries = self.byte_reader.read_u16()?;
        let mut frames = Vec::with_capacity(number_of_entries as usize);
        for _ in 0..number_of_entries {
            let frame = self.read_stack_map_frame()?;
            frames.push(frame);
        }

        Ok(Attribute::StackMapTable(StackMapTable::new(frames)))
    }

    fn read_stack_map_frame(&mut self) -> Result<StackMapFrame> {
        let frame_type = self.byte_reader.read_u8()?;
        match frame_type {
            0..=63 => Ok(StackMapFrame::SameFrame { frame_type }),
            64..=127 => {
                let stack = self.read_verification_type()?;
                Ok(StackMapFrame::SameLocals1StackItemFrame { frame_type, stack })
            }
            247 => {
                let offset_delta = self.byte_reader.read_u16()?;
                let stack = self.read_verification_type()?;
                Ok(StackMapFrame::SameLocals1StackItemFrameExtended { frame_type, offset_delta, stack })
            }
            248..=250 => {
                let offset_delta = self.byte_reader.read_u16()?;
                Ok(StackMapFrame::ChopFrame { frame_type, offset_delta })
            }
            251 => {
                let offset_delta = self.byte_reader.read_u16()?;
                Ok(StackMapFrame::SameFrameExtended { frame_type, offset_delta })
            }
            252..=254 => {
                let offset_delta = self.byte_reader.read_u16()?;
                let locals_count = frame_type - 251;
                let locals = self.read_verification_types(locals_count as u16)?;
                Ok(StackMapFrame::AppendFrame { frame_type, offset_delta, locals })
            }
            255 => {
                let offset_delta = self.byte_reader.read_u16()?;
                let locals_count = self.byte_reader.read_u16()?;
                let locals = self.read_verification_types(locals_count)?;
                let stack_count = self.byte_reader.read_u16()?;
                let stack = self.read_verification_types(stack_count)?;
                Ok(StackMapFrame::FullFrame { frame_type, offset_delta, locals, stack })
            }
            _ => return Err(ClassReaderError::InvalidStackMapFrameType(frame_type))
        }
    }

    fn read_verification_type(&mut self) -> Result<VerificationTypeInfo> {
        let tag = self.byte_reader.read_u8()?;
        Ok(match tag {
            0 => VerificationTypeInfo::Top,
            1 => VerificationTypeInfo::Integer,
            2 => VerificationTypeInfo::Float,
            3 => VerificationTypeInfo::Double,
            4 => VerificationTypeInfo::Long,
            5 => VerificationTypeInfo::Null,
            6 => VerificationTypeInfo::UninitializedThis,
            7 => {
                let cpool_index = self.byte_reader.read_u16()?;
                let constant = self.class_file.constant_pool.get(cpool_index as usize)?;
                VerificationTypeInfo::Object { constant: constant.clone() }
            }
            8 => {
                let offset = self.byte_reader.read_u16()?;
                VerificationTypeInfo::Uninitialized { offset }
            }
            _ => return Err(ClassReaderError::InvalidVerificationType(tag))
        })
    }

    fn read_verification_types(&mut self, count: u16) -> Result<Vec<VerificationTypeInfo>> {
        let mut types = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let verification_type = self.read_verification_type()?;
            types.push(verification_type);
        }
        Ok(types)
    }

    fn read_nest_members_attr(&mut self) -> Result<Attribute> {
        let attribute_name_index = self.byte_reader.read_u16()?;
        let _ = self.check_utf8(attribute_name_index, "NestMembers");
        let attribute_length = self.byte_reader.read_u32()?;
        let mut nest_members = Vec::new();
        for _ in 0..attribute_length {
            let class_index = self.byte_reader.read_u16()?;
            let class_name = self.get_class_name(class_index)?;
            nest_members.push(class_name);
        }
        Ok(NestMembers { names: nest_members }.into())
    }

    fn read_source_file_attr(&mut self) -> Result<Attribute> {
        let attribute_name_index = self.byte_reader.read_u16()?;
        match self.get_utf8(attribute_name_index) {
            Ok(string) => {
                if string != "SourceFile" {
                    return Err(ClassReaderError::InvalidSourceFileString(string))
                }
            }
            Err(err) => return Err(err.into())
        };
        let attribute_length = self.byte_reader.read_u32()?;
        if attribute_length != 2 {
            return Err(ClassReaderError::InvalidAttributeSize(attribute_length, 2));
        }
        let source_file_index = self.byte_reader.read_u16()?;
        let file_name = self.get_utf8(source_file_index)?;
        Ok(Attribute::SourceFile(SourceFile { file_name } ))
    }

    fn read_user_defined_attr(&mut self, name: String) -> Result<Attribute> {
        let length = self.byte_reader.read_u32()?;
        let info = self.byte_reader.read_bytes(length as usize)?;
        Ok(Attribute::UserDefined(UserDefinedAttribute::new(name, info)))
    }
}
