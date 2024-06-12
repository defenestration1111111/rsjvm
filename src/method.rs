use std::{iter::Peekable, str::Chars};

use crate::{attribute::Attribute, field::{FieldError, FieldType}};

type Result<T> = std::result::Result<T, MethodParsingError>;

#[derive(Debug, thiserror::Error)]
pub enum MethodParsingError {
    #[error("No opening bracket at the beginning of descriptor")]
    #[non_exhaustive]
    NoOpeningBracket,
    #[error("No closing bracket in descriptor")]
    #[non_exhaustive]
    NoClosingBracket,
    #[error("Error parsing field type: {0}")]
    #[non_exhaustive]
    FieldError(#[from] FieldError),
}

#[derive(Debug, Clone)]
pub struct Method {
    pub flags: MethodAccessFlags,
    pub name: String,
    pub type_descriptor: MethodDescriptor,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub enum MethodFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Bridge,
    Varargs,
    Native,
    Abstract,
    Strict,
    Synthetic,
}

#[derive(Debug, Clone)]
pub struct MethodAccessFlags {
    flags: Vec<MethodFlag>,
}

impl MethodAccessFlags {
    pub fn new(mask: u16) -> Self {
        let mut flags = Vec::new();

        if mask & 0x0001 != 0 {
            flags.push(MethodFlag::Public);
        }

        if mask & 0x0002 != 0 {
            flags.push(MethodFlag::Private);
        }

        if mask & 0x0004 != 0 {
            flags.push(MethodFlag::Protected);
        }

        if mask & 0x0008 != 0 {
            flags.push(MethodFlag::Static);
        }

        if mask & 0x0010 != 0 {
            flags.push(MethodFlag::Final);
        }

        if mask & 0x0020 != 0 {
            flags.push(MethodFlag::Synchronized);
        }

        if mask & 0x0040 != 0 {
            flags.push(MethodFlag::Bridge);
        }

        if mask & 0x0080 != 0 {
            flags.push(MethodFlag::Varargs);
        }

        if mask & 0x0100 != 0 {
            flags.push(MethodFlag::Native);
        }

        if mask & 0x0400 != 0 { 
            flags.push(MethodFlag::Abstract);
        }

        if mask & 0x0800 != 0 { 
            flags.push(MethodFlag::Strict);
        }

        if mask & 0x1000 != 0 { 
            flags.push(MethodFlag::Synthetic);
        }

        MethodAccessFlags { flags }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParameterDescriptor(Vec<FieldType>);

#[derive(Debug, PartialEq, Clone)]
pub struct MethodDescriptor(ParameterDescriptor, ReturnDescriptor);

impl MethodDescriptor {
    pub fn try_from(chars: &mut Peekable<Chars>) -> Result<MethodDescriptor> {
        if chars.next() != Some('(') {
            return Err(MethodParsingError::NoOpeningBracket)
        }

        let mut parameters = Vec::new();
        while let Some(&c) = chars.peek() {
            if c == ')' {
                break;
            }
            parameters.push(FieldType::try_from(chars)?);
        }

        if chars.next() != Some(')') {
            return Err(MethodParsingError::NoClosingBracket)
        }

        let return_type = match chars.peek() {
            Some(&'V') => {
                chars.next();
                ReturnDescriptor::VoidDescriptor
            }
            _ => ReturnDescriptor::FieldType(FieldType::try_from(chars)?)
        };
        Ok(MethodDescriptor(ParameterDescriptor(parameters), return_type))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnDescriptor {
    FieldType(FieldType),
    VoidDescriptor,
}

#[derive(Debug)]
pub struct VoidDescriptor;

#[cfg(test)]
mod tests {
    use crate::field::BaseType;

    use super::*;

    #[test]
    fn test_valid_method_descriptor() {
        let descriptor = "(IDLjava/lang/Thread;)Ljava/lang/Object;";
        let expected = MethodDescriptor(
            ParameterDescriptor(vec![
                FieldType::Base(BaseType::Int),
                FieldType::Base(BaseType::Double),
                FieldType::Object("java/lang/Thread".to_string()),
            ]),
            ReturnDescriptor::FieldType(FieldType::Object("java/lang/Object".to_string())),
        );

        let result = MethodDescriptor::try_from(&mut descriptor.chars().peekable());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_void_return_method_descriptor() {
        let descriptor = "(I)V";
        let expected = MethodDescriptor(
            ParameterDescriptor(vec![FieldType::Base(BaseType::Int)]),
            ReturnDescriptor::VoidDescriptor,
        );

        let result = MethodDescriptor::try_from(&mut descriptor.chars().peekable());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn no_parameters_method_descriptor_success() {
        let descriptor = "()V";
        let expected = MethodDescriptor(
            ParameterDescriptor(vec![]),
            ReturnDescriptor::VoidDescriptor,
        );

        let result = MethodDescriptor::try_from(&mut descriptor.chars().peekable());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn array_parameter_method_descriptor_success() {
        let descriptor = "([I)V";
        let expected = MethodDescriptor(
            ParameterDescriptor(vec![FieldType::Array(Box::new(FieldType::Base(BaseType::Int)))]),
            ReturnDescriptor::VoidDescriptor,
        );

        let result = MethodDescriptor::try_from(&mut descriptor.chars().peekable());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn invalid_descriptor_missing_opening_bracket() {
        let descriptor = "IDLjava/lang/Thread;)Ljava/lang/Object;";
        let result = MethodDescriptor::try_from(&mut descriptor.chars().peekable());
        assert!(matches!(result, Err(MethodParsingError::NoOpeningBracket)));
    }

    #[test]
    fn invalid_descriptor_missing_closing_bracket() {
        let descriptor = "(IDLjava/lang/Thread;Ljava/lang/Object;";
        let result = MethodDescriptor::try_from(&mut descriptor.chars().peekable());
        assert!(matches!(result, Err(MethodParsingError::NoClosingBracket)));
    }
}
