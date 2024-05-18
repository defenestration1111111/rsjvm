#[derive(Debug, PartialEq, Eq)]
enum AccessFlag {
    Public,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum,
    Module,
}

#[derive(Debug, Default)]
pub struct ClassFileAccessFlags {
    flags: Vec<AccessFlag>,
}

impl ClassFileAccessFlags {
    pub fn new(mask: u16) -> Self {
        let mut flags = Vec::new();

        if mask & 0x0001 != 0 {
            flags.push(AccessFlag::Public);
        }

        if mask & 0x0010 != 0 {
            flags.push(AccessFlag::Final);
        }

        if mask & 0x0020 != 0 {
            flags.push(AccessFlag::Super);
        }

        if mask & 0x0200 != 0 {
            flags.push(AccessFlag::Interface);
        }

        if mask & 0x0400 != 0 {
            flags.push(AccessFlag::Abstract);
        }

        if mask & 0x1000 != 0 {
            flags.push(AccessFlag::Synthetic);
        }

        if mask & 0x2000 != 0 {
            flags.push(AccessFlag::Annotation);
        }

        if mask & 0x4000 != 0 {
            flags.push(AccessFlag::Enum);
        }

        if mask & 0x8000 != 0 {
            flags.push(AccessFlag::Module);
        }

        ClassFileAccessFlags { flags }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_final_super_test() {
        let mask = 0x0031;
        let flags = ClassFileAccessFlags::new(mask).flags;

        assert_eq!(flags.len(), 3);
        assert!(flags.contains(&AccessFlag::Public));
        assert!(flags.contains(&AccessFlag::Final));
        assert!(flags.contains(&AccessFlag::Super));
    }
}
