use rsjvm::{class_file_reader::ClassFileReader, constant_pool::Constant};

#[test]
fn read_constant_pool() {
    let class_file = ClassFileReader::read_class(include_bytes!("resources/ConstantPool.class")).unwrap();

    assert_eq!(
        class_file.constant_pool.constants,
        [
            Constant::MethodRef(2, 3),
            Constant::ClassIndex(4),
            Constant::NameAndType(5, 6),
            Constant::Utf8("java/lang/Object".to_string()),
            Constant::Utf8("<init>".to_string()),
            Constant::Utf8("()V".to_string()),
            Constant::FieldRef(8, 9),
            Constant::ClassIndex(10),
            Constant::NameAndType(11, 12),
            Constant::Utf8("ConstantPool".to_string()),
            Constant::Utf8("nullable".to_string()),
            Constant::Utf8("LConstantPool;".to_string()),
            Constant::Utf8("integer".to_string()),
            Constant::Utf8("I".to_string()),
            Constant::Utf8("ConstantValue".to_string()),
            Constant::Integer(48),
            Constant::Utf8("string".to_string()),
            Constant::Utf8("Ljava/lang/String;".to_string()),
            Constant::Utf8("constantPool".to_string()),
            Constant::Utf8("Code".to_string()),
        ]
    );
}
