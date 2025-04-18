use common::{CompileConfig, check_javac_version};
use rsjvm_class_reader::attribute::Attribute;
use rsjvm_class_reader::class_file_reader::ClassFileReader;
use rsjvm_class_reader::predefined_attributes::PetrmittedSubclasses;

mod common;

#[test]
fn test_permitted_subclasses_attr() {
    if let Err(e) = check_javac_version() {
        panic!("{}", e);
    }

    let config = CompileConfig::new("PermittedSubclasses.java".to_string());
    let bytes = config.run();

    let class_file = ClassFileReader::read_class(&bytes.unwrap()).unwrap();

    let expected_permitted_subclasses = Attribute::PermittedSubclasses(PetrmittedSubclasses {
        names: vec!["Subclass".to_string(), "Subclass2".to_string()],
    });

    let actual_permitted_subclasses = class_file
        .attributes
        .iter()
        .find(|a| matches!(a, Attribute::PermittedSubclasses(_)))
        .expect("PermittedSubclasses attribute not found");

    assert_eq!(actual_permitted_subclasses, &expected_permitted_subclasses);
}
