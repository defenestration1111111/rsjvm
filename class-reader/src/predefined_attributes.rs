use derive_more::From;

use crate::attribute::Attribute;
use crate::constant_pool::Constant;
use crate::instruction::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantValue {
    value: Constant,
}

impl ConstantValue {
    pub fn new(constant_value: Constant) -> Self {
        ConstantValue { value: constant_value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<(Instruction, u32)>,
    pub exception_table: Vec<ExceptionHandler>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionHandler {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionHandler {
    pub fn new(start_pc: u16, end_pc: u16, handler_pc: u16, catch_type: u16) -> Self {
        ExceptionHandler { start_pc, end_pc, handler_pc, catch_type }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineNumberTable {
    pub line_number_table: Vec<LineNumber>,
}

impl LineNumberTable {
    pub fn new(line_number_table: Vec<LineNumber>) -> LineNumberTable {
        LineNumberTable { line_number_table }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTable {
    local_variable_table: Vec<LocalVariable>,
}

impl LocalVariableTable {
    pub fn new(local_variable_table: Vec<LocalVariable>) -> LocalVariableTable {
        LocalVariableTable { local_variable_table }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl LocalVariable {
    pub fn new(
        start_pc: u16,
        length: u16,
        name_index: u16,
        descriptor_index: u16,
        index: u16,
    ) -> Self {
        Self { start_pc, length, name_index, descriptor_index, index }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTypeTable {
    local_variable_type_table: Vec<LocalVariableType>,
}

impl LocalVariableTypeTable {
    pub fn new(local_variable_type_table: Vec<LocalVariableType>) -> Self {
        Self { local_variable_type_table }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableType {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

impl LocalVariableType {
    pub fn new(
        start_pc: u16,
        length: u16,
        name_index: u16,
        signature_index: u16,
        index: u16,
    ) -> Self {
        Self { start_pc, length, name_index, signature_index, index }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackMapTable {
    frames: Vec<StackMapFrame>,
}

impl StackMapTable {
    pub fn new(frames: Vec<StackMapFrame>) -> StackMapTable {
        StackMapTable { frames }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: u8, /* 0-63 */
    },
    SameLocals1StackItemFrame {
        frame_type: u8, /* 64-127 */
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFrameExtended {
        frame_type: u8, /* 247 */
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        frame_type: u8, /* 248-250 */
        offset_delta: u16,
    },
    SameFrameExtended {
        frame_type: u8, /* 251 */
        offset_delta: u16,
    },
    AppendFrame {
        frame_type: u8, /* 252-254 */
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        frame_type: u8, /* 255 */
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethods {
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

impl BootstrapMethods {
    pub fn new(bootstrap_methods: Vec<BootstrapMethod>) -> BootstrapMethods {
        BootstrapMethods { bootstrap_methods }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, Clone, From, PartialEq)]
pub struct NestHost {
    pub name: String,
}

#[derive(Debug, Clone, From, PartialEq)]
pub struct NestMembers {
    pub names: Vec<String>,
}

#[derive(Debug, Clone, From, PartialEq)]
pub struct PetrmittedSubclasses {
    pub names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { constant: Constant },
    Uninitialized { offset: u16 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceFile {
    pub file_name: String,
}
