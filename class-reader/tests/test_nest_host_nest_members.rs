use std::path::Path;

use common::{check_javac_version, read_class_file, CompileConfig};
use rsjvm_class_reader::attribute::Attribute;
use rsjvm_class_reader::class_file_reader::ClassFileReader;
use rsjvm_class_reader::predefined_attributes::{NestHost, NestMembers};

mod common;

#[test]
fn test_nest_host_nest_members_attrs() {
    if let Err(e) = check_javac_version() {
        panic!("{}", e);
    }

    let config = CompileConfig::new("NestHostNestMembers.java".to_string());
    let bytes = config.run();

    let host_file = ClassFileReader::read_class(&bytes.unwrap()).unwrap();
    let member_file = ClassFileReader::read_class(
        &read_class_file(Path::new("target/classes/NestHostNestMembers$Inner.class")).unwrap(),
    )
    .unwrap();

    let expected_nest_members = Attribute::NestMembers(NestMembers {
        names: vec!["NestHostNestMembers$Inner".to_string()],
    });
    let expected_nest_host =
        Attribute::NestHost(NestHost { name: "NestHostNestMembers".to_string() });

    let actual_nest_members = host_file
        .attributes
        .iter()
        .find(|a| matches!(a, Attribute::NestMembers(_)))
        .expect("NestMembers attribute not found");

    let actual_nest_host = member_file
        .attributes
        .iter()
        .find(|a| matches!(a, Attribute::NestHost(_)))
        .expect("NestHost attribute not found");

    assert_eq!(actual_nest_members, &expected_nest_members);
    assert_eq!(actual_nest_host, &expected_nest_host);
}
