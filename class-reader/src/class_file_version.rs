use std::fmt::Display;

type Result<T> = std::result::Result<T, FileVersionError>;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FileVersionError {
    #[error("Unsupported major version {0}")]
    #[non_exhaustive]
    UnsupportedMajorVersion(u16),
    #[error("Unsupported minor version {1} for major version {0}")]
    #[non_exhaustive]
    UnsupportedMinorVersion(u16, u16),
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct ClassFileVersion(MajorVersion, u16);

impl ClassFileVersion {
    pub fn from(major: u16, minor: u16) -> Result<ClassFileVersion> {
        use MajorVersion::*;

        let major_version = MajorVersion::try_from(major)?;

        let is_invalid_minor = (major_version == JavaSE_1_1 && minor >= 3)
            || (major_version < JavaSE_12 && minor != 0)
            || (minor != 0 && minor != 65535);

        if is_invalid_minor {
            Err(FileVersionError::UnsupportedMinorVersion(major, minor))
        } else {
            Ok(ClassFileVersion(major_version, minor))
        }
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, strum_macros::Display)]
#[allow(non_camel_case_types)]
pub enum MajorVersion {
    JavaSE_1_1,
    JavaSE_1_2,
    JavaSE_1_3,
    JavaSE_1_4,
    JavaSE_5_0,
    JavaSE_6,
    JavaSE_7,
    JavaSE_8,
    JavaSE_9,
    JavaSE_10,
    JavaSE_11,
    JavaSE_12,
    JavaSE_13,
    JavaSE_14,
    JavaSE_15,
    JavaSE_16,
    #[default]
    JavaSE_17,
    JavaSE_18,
    JavaSE_19,
    JavaSE_20,
    JavaSE_21,
    JavaSE_22,
    JavaSE_23,
}

impl TryFrom<u16> for MajorVersion {
    type Error = FileVersionError;

    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        use MajorVersion::*;

        match value {
            45 => Ok(JavaSE_1_1),
            46 => Ok(JavaSE_1_2),
            47 => Ok(JavaSE_1_3),
            48 => Ok(JavaSE_1_4),
            49 => Ok(JavaSE_5_0),
            50 => Ok(JavaSE_6),
            51 => Ok(JavaSE_7),
            52 => Ok(JavaSE_8),
            53 => Ok(JavaSE_9),
            54 => Ok(JavaSE_10),
            55 => Ok(JavaSE_11),
            56 => Ok(JavaSE_12),
            57 => Ok(JavaSE_13),
            58 => Ok(JavaSE_14),
            59 => Ok(JavaSE_15),
            60 => Ok(JavaSE_16),
            61 => Ok(JavaSE_17),
            62 => Ok(JavaSE_18),
            63 => Ok(JavaSE_19),
            64 => Ok(JavaSE_20),
            65 => Ok(JavaSE_21),
            66 => Ok(JavaSE_22),
            67 => Ok(JavaSE_23),
            _ => Err(FileVersionError::UnsupportedMajorVersion(value)),
        }
    }
}

impl Display for ClassFileVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Major version: {}, minor version: {}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::class_file_version::{ClassFileVersion, FileVersionError, MajorVersion};

    #[test]
    fn test_major_success() {
        assert_eq!(MajorVersion::try_from(50), Ok(MajorVersion::JavaSE_6));
    }

    #[test]
    fn test_major_error() {
        assert_eq!(
            MajorVersion::try_from(777),
            Err(FileVersionError::UnsupportedMajorVersion(777))
        );
    }

    #[test]
    fn test_minor_success() {
        assert!((ClassFileVersion::from(45, 3).is_err()));
    }

    #[test]
    fn test_minor_unsupported_minor_error() {
        assert_eq!(
            ClassFileVersion::from(65, 555),
            Err(FileVersionError::UnsupportedMinorVersion(65, 555))
        );
    }
}
