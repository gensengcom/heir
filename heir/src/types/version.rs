use std::fmt;
use std::io::{self, Read, Write};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Version(u8);

impl Version {
    /// Constructs a [`Version`].
    pub fn new(major: u8, minor: u8) -> Result<Self, VersionError> {
        if major >= 16 {
            return Err(VersionError::MajorVersionExceedsRange(major));
        }
        if minor >= 16 {
            return Err(VersionError::MinorVersionExceedsRange(minor));
        }
        Ok(Version((major << 4) | (minor & 0x0F)))
    }

    /// Constructs a compact [`Version`] without checking input bounds.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `major` and `minor` are below 16.
    pub unsafe fn new_unchecked(major: u8, minor: u8) -> Self {
        Version((major << 4) | (minor & 0x0F))
    }

    /// Retrieves the major version number.
    pub fn major(&self) -> u8 {
        self.0 >> 4
    }

    /// Retrieves the minor version number.
    pub fn minor(&self) -> u8 {
        self.0 & 0x0F
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&[self.0])
    }

    /// Deserializes a `Version` from the given reader.
    pub fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let version = Version(buf[0]);
        // Validate the major and minor versions
        if version.major() >= 16 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                VersionError::MajorVersionExceedsRange(version.major()),
            ));
        }
        if version.minor() >= 16 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                VersionError::MinorVersionExceedsRange(version.minor()),
            ));
        }
        Ok(version)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionError {
    MinorVersionExceedsRange(u8),
    MajorVersionExceedsRange(u8),
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionError::MajorVersionExceedsRange(major) => {
                write!(f, "Major version {} is beyond range [0,16).", major)
            }
            VersionError::MinorVersionExceedsRange(minor) => {
                write!(f, "Minor version {} is beyond range [0,16).", minor)
            }
        }
    }
}

impl std::error::Error for VersionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_new_valid() {
        // Test valid major and minor versions
        let version = Version::new(1, 0).expect("Valid version");
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 0);

        let version = Version::new(15, 15).expect("Valid version");
        assert_eq!(version.major(), 15);
        assert_eq!(version.minor(), 15);

        let version = Version::new(0, 0).expect("Valid version");
        assert_eq!(version.major(), 0);
        assert_eq!(version.minor(), 0);
    }

    #[test]
    fn test_version_new_invalid_major() {
        // Test invalid major version (>=16)
        let result = Version::new(16, 0);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, VersionError::MajorVersionExceedsRange(16));
        }

        let result = Version::new(20, 5);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, VersionError::MajorVersionExceedsRange(20));
        }
    }

    #[test]
    fn test_version_new_invalid_minor() {
        // Test invalid minor version (>=16)
        let result = Version::new(0, 16);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, VersionError::MinorVersionExceedsRange(16));
        }

        let result = Version::new(5, 20);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, VersionError::MinorVersionExceedsRange(20));
        }
    }

    #[test]
    fn test_version_serialize_deserialize() {
        let original_version = Version::new(3, 5).expect("Valid version");
        let mut buffer = Vec::new();
        original_version
            .serialize(&mut buffer)
            .expect("Serialization failed");

        let mut cursor = &buffer[..];
        let deserialized_version =
            Version::deserialize(&mut cursor).expect("Deserialization failed");

        assert_eq!(original_version, deserialized_version);
        assert_eq!(deserialized_version.major(), 3);
        assert_eq!(deserialized_version.minor(), 5);
    }

    #[test]
    fn test_version_deserialize_invalid_data() {
        // Invalid major version during deserialization
        let invalid_data = [0xF0]; // Major version 15, minor version 0
        let mut cursor = &invalid_data[..];
        let version = Version::deserialize(&mut cursor).expect("Deserialization should succeed");
        assert_eq!(version.major(), 15);
        assert_eq!(version.minor(), 0);

        let invalid_data = [0xFF]; // Major version 15, minor version 15
        let mut cursor = &invalid_data[..];
        let version = Version::deserialize(&mut cursor).expect("Deserialization should succeed");
        assert_eq!(version.major(), 15);
        assert_eq!(version.minor(), 15);

        // Invalid data (major version >=16)
        let invalid_data = [0xF1]; // Major version 15, minor version 1
        let mut cursor = &invalid_data[..];
        let version = Version::deserialize(&mut cursor).expect("Deserialization should succeed");
        assert_eq!(version.major(), 15);
        assert_eq!(version.minor(), 1);

        let invalid_data = [0x10]; // Major version 1, minor version 0
        let mut cursor = &invalid_data[..];
        let version = Version::deserialize(&mut cursor).expect("Deserialization should succeed");
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 0);

        // Invalid major version
        let invalid_data = [0xF0]; // Major version 15 (valid), minor version 0
        let mut cursor = &invalid_data[..];
        let version = Version::deserialize(&mut cursor).expect("Deserialization should succeed");
        assert_eq!(version.major(), 15);
        assert_eq!(version.minor(), 0);
    }

    #[test]
    fn test_version_display() {
        let version = Version::new(2, 3).expect("Valid version");
        assert_eq!(format!("{:?}", version), "Version(35)");
    }

    #[test]
    fn test_version_new_unchecked() {
        unsafe {
            let version = Version::new_unchecked(1, 0);
            assert_eq!(version.major(), 1);
            assert_eq!(version.minor(), 0);

            let version = Version::new_unchecked(15, 15);
            assert_eq!(version.major(), 15);
            assert_eq!(version.minor(), 15);
        }
    }
}
