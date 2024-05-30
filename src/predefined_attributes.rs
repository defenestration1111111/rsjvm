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

#[derive(Debug)]
pub struct StackMapTable {
    frames: Vec<StackMapFrame>,
}

impl StackMapTable {
    pub fn new(frames: Vec<StackMapFrame>) -> StackMapTable {
        StackMapTable { frames }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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