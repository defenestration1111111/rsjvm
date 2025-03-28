mod common;

use common::{CompileConfig, check_javac_version};
use rsjvm_class_reader::attribute::Attribute;
use rsjvm_class_reader::class_file_reader::ClassFileReader;
use rsjvm_class_reader::constant_pool::Constant;
use rsjvm_class_reader::instruction::Instruction;
use rsjvm_class_reader::predefined_attributes::{
    Code, LineNumber, LineNumberTable, LocalVariable, LocalVariableTable, LocalVariableType,
    LocalVariableTypeTable, StackMapFrame, StackMapTable, VerificationTypeInfo,
};

#[test]
fn test_read_code_with_attrs() {
    if let Err(e) = check_javac_version() {
        panic!("{}", e);
    }

    let config = CompileConfig::new("IntListControlFlowSingleFunction.java".to_string());
    let bytes = config.run();

    let class_file = ClassFileReader::read_class(&bytes.unwrap()).unwrap();

    let line_number_table_attr = LineNumberTable::new(vec![
        LineNumber { start_pc: 0, line_number: 6 },
        LineNumber { start_pc: 8, line_number: 7 },
        LineNumber { start_pc: 17, line_number: 8 },
        LineNumber { start_pc: 28, line_number: 10 },
    ]);

    let local_variable_table_attr = LocalVariableTable::new(vec![
        LocalVariable::new(0, 29, 31, 32, 0),
        LocalVariable::new(8, 21, 34, 35, 1),
    ]);

    let local_variable_type_table_attr =
        LocalVariableTypeTable::new(vec![LocalVariableType::new(8, 21, 34, 37, 1)]);

    let stack_map_table_attr = StackMapTable::new(vec![StackMapFrame::AppendFrame {
        frame_type: 252,
        offset_delta: 28,
        locals: vec![VerificationTypeInfo::Object { constant: Constant::ClassIndex(13) }],
    }]);

    let expected_code_attr = Attribute::Code(Code {
        max_stack: 2,
        max_locals: 2,
        code: vec![
            (Instruction::New(7), 0),
            (Instruction::Dup, 3),
            (Instruction::Invokespecial(9), 4),
            (Instruction::Astore_1, 7),
            (Instruction::Aload_1, 8),
            (Instruction::Invokeinterface(10, 1), 9),
            (Instruction::Nop, 13),
            (Instruction::Iflt(14), 14),
            (Instruction::Aload_1, 17),
            (Instruction::Iconst_1, 18),
            (Instruction::Invokestatic(16), 19),
            (Instruction::Invokeinterface(22, 2), 22),
            (Instruction::Nop, 26),
            (Instruction::Pop, 27),
            (Instruction::Return, 28),
        ],
        exception_table: vec![],
        attributes: vec![
            Attribute::LineNumberTable(line_number_table_attr),
            Attribute::LocalVariableTable(local_variable_table_attr),
            Attribute::LocalVariableTypeTable(local_variable_type_table_attr),
            Attribute::StackMapTable(stack_map_table_attr),
        ],
    });

    let method = class_file
        .methods
        .iter()
        .find(|m| m.name == "function")
        .expect("Method 'function' not found");

    let code_attr = method
        .attributes
        .iter()
        .find(|attr| matches!(attr, Attribute::Code(_)))
        .expect("Code attribute not found in method 'function'");

    if let (Attribute::Code(actual_code), Attribute::Code(expected_code)) =
        (code_attr, expected_code_attr)
    {
        if *actual_code != expected_code {
            panic!("Field mismatch!\nExpected: {:?}\nActual: {:?}", expected_code, actual_code);
        }
    } else {
        panic!("Expected a Code attribute but found something else.");
    }
}
