use std::{iter::{from_fn, Peekable}, str::Chars};

use crate::attribute::Attribute;

#[derive(Debug, thiserror::Error)]
pub enum FieldError {
    #[error("End of characters is not expected")]
    #[non_exhaustive]
    UnexpectedEnd,
    #[error("No semicolon at the end object type descriptor")]
    #[non_exhaustive]
    NoSemicolon,
    #[error("Error while parsing the descriptor")]
    #[non_exhaustive]
    InvalidDescriptor,
}


#[derive(Debug, Clone)]
pub struct Field {
    flags: FieldAccessFlags,
    name: String,
    type_descriptor: FieldType,
    attributes: Vec<Attribute>,
}

impl Field {
    pub fn new(flags: FieldAccessFlags, name: String, type_descriptor: FieldType, attributes: Vec<Attribute>) -> Self {
        Field { flags, name, type_descriptor, attributes }
    }
}
#[derive(Debug, Clone)]
pub enum AccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Volatile,
    Transient,
    Synthetic,
    Enum,
}

#[derive(Debug, Clone)]
pub struct FieldAccessFlags {
    flags: Vec<AccessFlag>,
}

impl FieldAccessFlags {
    pub fn new(mask: u16) -> Self {
        let mut flags = Vec::new();

        if mask & 0x0001 != 0 {
            flags.push(AccessFlag::Public);
        }

        if mask & 0x0002 != 0 {
            flags.push(AccessFlag::Private);
        }

        if mask & 0x0004 != 0 {
            flags.push(AccessFlag::Protected);
        }

        if mask & 0x0008 != 0 {
            flags.push(AccessFlag::Static);
        }

        if mask & 0x0010 != 0 {
            flags.push(AccessFlag::Final);
        }

        if mask & 0x0040 != 0 {
            flags.push(AccessFlag::Volatile);
        }

        if mask & 0x0080 != 0 {
            flags.push(AccessFlag::Transient);
        }

        if mask & 0x1000 != 0 {
            flags.push(AccessFlag::Synthetic);
        }

        if mask & 0x4000 != 0 {
            flags.push(AccessFlag::Enum);
        }

        
        FieldAccessFlags { flags }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BaseType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldType {
    Base(BaseType),
    Object(String),
    Array(Box<FieldType>),
}

impl FieldType {
    pub fn try_from(chars: &mut Peekable<Chars>) -> Result<FieldType, FieldError> {
        match chars.next().ok_or(FieldError::UnexpectedEnd)? {
            'B' => Ok(FieldType::Base(BaseType::Byte)),
            'C' => Ok(FieldType::Base(BaseType::Char)),
            'D' => Ok(FieldType::Base(BaseType::Double)),
            'F' => Ok(FieldType::Base(BaseType::Float)),
            'I' => Ok(FieldType::Base(BaseType::Int)),
            'J' => Ok(FieldType::Base(BaseType::Long)),
            'S' => Ok(FieldType::Base(BaseType::Short)),
            'Z' => Ok(FieldType::Base(BaseType::Boolean)),
            'L' => {
                let class_name: String = from_fn(|| chars.next_if(|&ch| ch != ';')).collect();
                if chars.next().is_some() {
                    Ok(FieldType::Object(class_name))
                } else {
                    Err(FieldError::NoSemicolon)
                }
            }
            '[' => {
                let element_type = FieldType::try_from(chars)?;
                Ok(FieldType::Array(Box::new(element_type)))
            }
            _ => Err(FieldError::InvalidDescriptor)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::field::*;

    #[test]
    fn boolean_success() {
        assert_eq!(
            FieldType::try_from(&mut "Z".chars().peekable()).unwrap(),
            FieldType::Base(BaseType::Boolean)
        );
    }

    #[test]
    fn array_of_objects_success() {
        assert_eq!(
            FieldType::try_from(&mut "[Ljava/lang/String;".chars().peekable()).unwrap(),
            FieldType::Array(Box::new(FieldType::Object("java/lang/String".to_string())))
        );
    }

    #[test]
    fn int_array_multidimensional_success() {
        assert_eq!(
            FieldType::try_from(&mut "[[[I".chars().peekable()).unwrap(),
            FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Base(BaseType::Int)))))))
        );
    }
}