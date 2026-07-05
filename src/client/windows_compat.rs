pub mod xattr {
    use std::{
        io,
        path::Path,
    };

    /// Implementing xattr-like feature on Windows using Alternate Data Streams(ADS)
    /// ADS only works on NTFS filesystem, and maybe removed in specific operations.
    /// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/e2b19412-a925-4360-b009-86e3b8a020c8
    ///
    /// A stream is addressed through the regular file API as `<path>:<attr_name>`.
    pub fn get_xattr(path: &Path, attr_name: &str) -> io::Result<String> {
        let stream = format!("{}:{}", path.display(), attr_name);
        let bytes = std::fs::read(stream)?;
        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn set_xattr(path: &Path, attr_name: &str, value: String) -> io::Result<()> {
        let stream = format!("{}:{}", path.display(), attr_name);
        std::fs::write(stream, value)
    }
}
