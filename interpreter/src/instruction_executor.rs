#![allow(dead_code)]

use rsjvm_class_reader::instruction::Instruction::{self, *};

use crate::frame::{Frame, Value, binary_op};

struct InstructionExecutor;

impl InstructionExecutor {
    pub fn execute(instruction: Instruction, frame: &mut Frame) {
        match instruction {
            Aconst_null => frame.push_operand(Value::Null),
            Dconst_0 => frame.push_operand(Value::Double(0.0)),
            Dconst_1 => frame.push_operand(Value::Double(1.0)),
            Fconst_0 => frame.push_operand(Value::Float(0.0)),
            Fconst_1 => frame.push_operand(Value::Float(1.0)),
            Fconst_2 => frame.push_operand(Value::Float(2.0)),
            Iand => binary_op::<i32, _>(|a, b| a & b, frame),
            Iconst_m1 => frame.push_operand(Value::Int(-1)),
            Iconst_0 => frame.push_operand(Value::Int(0)),
            Iconst_1 => frame.push_operand(Value::Int(1)),
            Iconst_2 => frame.push_operand(Value::Int(2)),
            Iconst_3 => frame.push_operand(Value::Int(3)),
            Iconst_4 => frame.push_operand(Value::Int(4)),
            Iconst_5 => frame.push_operand(Value::Int(5)),
            Ior => binary_op::<i32, _>(|a, b| a | b, frame),
            Ishl => binary_op::<i32, _>(|a, b| a << (b & 0x1F), frame),
            Ishr => binary_op::<i32, _>(|a, b| a >> (b & 0x1F), frame),
            Iushr => binary_op::<i32, _>(
                |a, b| {
                    if a >= 0 { a >> (b & 0x1F) } else { (a >> (b & 0x1F)) + (2 << !b) }
                },
                frame,
            ),
            Ixor => binary_op::<i32, _>(|a, b| a ^ b, frame),
            Land => binary_op::<i64, _>(|a, b| a & b, frame),
            Lconst_0 => {
                frame.push_operand(Value::Long(0));
            }
            Lconst_1 => {
                frame.push_operand(Value::Long(1));
            }
            Lor => binary_op::<i64, _>(|a, b| a | b, frame),
            Lshl => binary_op::<i64, _>(|a, b| a << (b & 0x3F), frame),
            Lshr => binary_op::<i64, _>(|a, b| a >> (b & 0x3F), frame),
            Lushr => binary_op::<i64, _>(
                |a, b| {
                    if a >= 0 { a >> (b & 0x3F) } else { (a >> (b & 0x3F)) + (2 << !b) }
                },
                frame,
            ),
            Lxor => binary_op::<i64, _>(|a, b| a ^ b, frame),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::frame::{Frame, Value};
    use crate::instruction_executor::InstructionExecutor;

    fn setup_frame() -> Frame {
        Frame::new(0, 2)
    }

    macro_rules! param_test {
        ($($name:ident: ($instr:expr, $a:expr, $b:expr) => $expected:expr),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut frame = setup_frame();
                    frame.push_operand(Value::Int($a));
                    frame.push_operand(Value::Int($b));
                    InstructionExecutor::execute($instr, &mut frame);
                    let result = frame.pop_operand();

                    match result {
                        Value::Int(value) => assert_eq!(value, $expected, "Failed for instruction {:?} with inputs ({}, {})", $instr, $a, $b),
                        _ => panic!("Expected Value::Int but got {:?}", result),
                    }
                }
            )*
        };
    }

    param_test! {
        test_ishr_16_2: (super::Ishr, 16, 2) => 4, // 16 >> 2 = 4

        test_iushr_16_2: (super::Iushr, 16, 2) => 4, // 16 >>> 2 = 4
        // test_iushr_neg16_2: (super::Iushr, -16, 2) => 1073741820, // -16 >>> 2 = 1073741820
        test_iushr_5_0: (super::Iushr, 5, 0) => 5, // 5 >>> 0 = 5
        // test_iushr_neg1_31: (super::Iushr, -1, 31) => 1, // -1 >>> 31 = 1
    }
}
