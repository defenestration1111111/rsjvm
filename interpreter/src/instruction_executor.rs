use rsjvm_class_reader::instruction::Instruction::{self, *};

use crate::frame::{Frame, Value};

struct InstructionExecutor;

impl InstructionExecutor {
    pub fn execute(instruction: Instruction, frame: &mut Frame) {
        match instruction {
            Aconst_null => {
                frame.push_operand(Value::Null);
            },
            Dconst_0 => {
                frame.push_operand(Value::Double(0.0));
             },  
            Dconst_1 => {
                frame.push_operand(Value::Double(1.0));
            },  
            Fconst_0 => {
                frame.push_operand(Value::Float(0.0));
            },
            Fconst_1 => {
                frame.push_operand(Value::Float(1.0));
            },
            Fconst_2 => {
                frame.push_operand(Value::Float(2.0));
            },  
            Iconst_m1 => {
                frame.push_operand(Value::Int(-1));
            },  
            Iconst_0 => {
                frame.push_operand(Value::Int(0));
            },  
            Iconst_1 => {
                frame.push_operand(Value::Int(1));
            },  
            Iconst_2 => { 
                frame.push_operand(Value::Int(2));
            },  
            Iconst_3 => {
                frame.push_operand(Value::Int(3));
            },  
            Iconst_4 => {
                frame.push_operand(Value::Int(4));
            },  
            Iconst_5 => {
                frame.push_operand(Value::Int(5));
            },  
            Lconst_0 => {
                frame.push_operand(Value::Long(0));
            },  
            Lconst_1 => {
                frame.push_operand(Value::Long(1));
            },  
            _ => unimplemented!(),
        }
    }
}
