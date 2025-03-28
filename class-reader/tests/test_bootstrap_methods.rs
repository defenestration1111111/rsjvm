use common::{CompileConfig, check_javac_version};
use rsjvm_class_reader::attribute::Attribute;
use rsjvm_class_reader::class_file_reader::ClassFileReader;
use rsjvm_class_reader::predefined_attributes::{BootstrapMethod, BootstrapMethods};

mod common;

#[test]
fn test_bootstrap_methods_attr() {
    if let Err(e) = check_javac_version() {
        panic!("{}", e);
    }

    let config = CompileConfig::new("BootstrapMethods.java".to_string());
    let bytes = config.run();

    let class_file = ClassFileReader::read_class(&bytes.unwrap()).unwrap();

    let expected_attr = BootstrapMethods::new(vec![BootstrapMethod {
        bootstrap_method_ref: 32,
        bootstrap_arguments: vec![28, 29, 28],
    }]);

    let expected_bootstrap_methods_attr = Attribute::BootstrapMethods(expected_attr);

    let actual_attr = class_file
        .attributes
        .iter()
        .find(|a| matches!(a, Attribute::BootstrapMethods(_)))
        .expect("BootstrapMethods attribute not found");

    assert_eq!(actual_attr, &expected_bootstrap_methods_attr);
}
