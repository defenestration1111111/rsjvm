use crate::{attribute::Attribute, constant_pool::Constant, instruction::Instruction};
use derive_more::From;

#[derive(Debug, Clone)]
pub struct ConstantValue {
    value: Constant,
}

impl ConstantValue {
    pub fn new(constant_value: Constant) -> Self {
        ConstantValue { value: constant_value }
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<(Instruction, u32)>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug, Clone)]
pub struct StackMapTable {
    frames: Vec<StackMapFrame>,
}

impl StackMapTable {
    pub fn new(frames: Vec<StackMapFrame>) -> StackMapTable {
        StackMapTable { frames }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, From)]
pub struct NestMembers {
    pub names: Vec<String>,
} 

#[derive(Debug, Clone, From)]
pub struct PetrmittedSubclasses {
    pub names: Vec<String>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub file_name: String,
}

}