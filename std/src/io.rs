use std::fs::{self, File as StdFile};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::fmt;

/// Error type for I/O operations
#[derive(Debug)]
pub enum IOError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    InvalidInput,
    UnexpectedEof,
    Other(String),
}

impl From<io::Error> for IOError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => IOError::NotFound,
            io::ErrorKind::PermissionDenied => IOError::PermissionDenied,
            io::ErrorKind::AlreadyExists => IOError::AlreadyExists,
            io::ErrorKind::InvalidInput => IOError::InvalidInput,
            io::ErrorKind::UnexpectedEof => IOError::UnexpectedEof,
            _ => IOError::Other(error.to_string()),
        }
    }
}

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOError::NotFound => write!(f, "File or directory not found"),
            IOError::PermissionDenied => write!(f, "Permission denied"),
            IOError::AlreadyExists => write!(f, "File or directory already exists"),
            IOError::InvalidInput => write!(f, "Invalid input"),
            IOError::UnexpectedEof => write!(f, "Unexpected end of file"),
            IOError::Other(message) => write!(f, "{}", message),
        }
    }
}

/// Result type for I/O operations
pub type IOResult<T> = Result<T, IOError>;

/// File system operations
pub struct FileSystem;

impl FileSystem {
    /// Check if a file exists
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).is_file()
    }
    
    /// Check if a directory exists
    pub fn directory_exists(path: &str) -> bool {
        Path::new(path).is_dir()
    }
    
    /// Create a directory
    pub fn create_directory(path: &str) -> IOResult<()> {
        fs::create_dir(path).map_err(IOError::from)
    }
    
    /// Create a directory and all parent directories
    pub fn create_directory_all(path: &str) -> IOResult<()> {
        fs::create_dir_all(path).map_err(IOError::from)
    }
    
    /// Remove a file
    pub fn remove_file(path: &str) -> IOResult<()> {
        fs::remove_file(path).map_err(IOError::from)
    }
    
    /// Remove a directory
    pub fn remove_directory(path: &str) -> IOResult<()> {
        fs::remove_dir(path).map_err(IOError::from)
    }
    
    /// Remove a directory and all its contents
    pub fn remove_directory_all(path: &str) -> IOResult<()> {
        fs::remove_dir_all(path).map_err(IOError::from)
    }
    
    /// Rename a file or directory
    pub fn rename(from: &str, to: &str) -> IOResult<()> {
        fs::rename(from, to).map_err(IOError::from)
    }
    
    /// Copy a file
    pub fn copy_file(from: &str, to: &str) -> IOResult<u64> {
        fs::copy(from, to).map_err(IOError::from)
    }
    
    /// Get the current working directory
    pub fn current_directory() -> IOResult<String> {
        let path = std::env::current_dir().map_err(IOError::from)?;
        Ok(path.to_string_lossy().to_string())
    }
    
    /// Change the current working directory
    pub fn set_current_directory(path: &str) -> IOResult<()> {
        std::env::set_current_dir(path).map_err(IOError::from)
    }
    
    /// List directory contents
    pub fn list_directory(path: &str) -> IOResult<Vec<String>> {
        let entries = fs::read_dir(path).map_err(IOError::from)?;
        let mut result = Vec::new();
        
        for entry in entries {
            let entry = entry.map_err(IOError::from)?;
            let path = entry.path();
            if let Some(name) = path.file_name() {
                result.push(name.to_string_lossy().to_string());
            }
        }
        
        Ok(result)
    }
}

/// File open options
pub struct FileOpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl FileOpenOptions {
    /// Create new file open options
    pub fn new() -> Self {
        FileOpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }
    
    /// Open for reading
    pub fn read(mut self, read: bool) -> Self {
        self.read = read;
        self
    }
    
    /// Open for writing
    pub fn write(mut self, write: bool) -> Self {
        self.write = write;
        self
    }
    
    /// Open for appending
    pub fn append(mut self, append: bool) -> Self {
        self.append = append;
        self
    }
    
    /// Truncate the file
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
    
    /// Create the file if it doesn't exist
    pub fn create(mut self, create: bool) -> Self {
        self.create = create;
        self
    }
    
    /// Create the file, failing if it already exists
    pub fn create_new(mut self, create_new: bool) -> Self {
        self.create_new = create_new;
        self
    }
    
    /// Open the file with the specified options
    pub fn open(&self, path: &str) -> IOResult<File> {
        let mut options = std::fs::OpenOptions::new();
        options
            .read(self.read)
            .write(self.write)
            .append(self.append)
            .truncate(self.truncate)
            .create(self.create)
            .create_new(self.create_new);
        
        let file = options.open(path).map_err(IOError::from)?;
        Ok(File { inner: file })
    }
}

/// File handle for reading and writing
pub struct File {
    inner: StdFile,
}

impl File {
    /// Open a file for reading
    pub fn open(path: &str) -> IOResult<Self> {
        let file = StdFile::open(path).map_err(IOError::from)?;
        Ok(File { inner: file })
    }
    
    /// Create a new file for writing
    pub fn create(path: &str) -> IOResult<Self> {
        let file = StdFile::create(path).map_err(IOError::from)?;
        Ok(File { inner: file })
    }
    
    /// Open a file with custom options
    pub fn with_options() -> FileOpenOptions {
        FileOpenOptions::new()
    }
    
    /// Read the entire file into a string
    pub fn read_to_string(&mut self) -> IOResult<String> {
        let mut string = String::new();
        self.inner.read_to_string(&mut string).map_err(IOError::from)?;
        Ok(string)
    }
    
    /// Read the entire file into a byte vector
    pub fn read_to_bytes(&mut self) -> IOResult<Vec<u8>> {
        let mut bytes = Vec::new();
        self.inner.read_to_end(&mut bytes).map_err(IOError::from)?;
        Ok(bytes)
    }
    
    /// Read up to `buf.len()` bytes into `buf`
    pub fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        self.inner.read(buf).map_err(IOError::from)
    }
    
    /// Write a string to the file
    pub fn write_string(&mut self, s: &str) -> IOResult<usize> {
        self.inner.write(s.as_bytes()).map_err(IOError::from)
    }
    
    /// Write bytes to the file
    pub fn write_bytes(&mut self, bytes: &[u8]) -> IOResult<usize> {
        self.inner.write(bytes).map_err(IOError::from)
    }
    
    /// Flush buffered data to disk
    pub fn flush(&mut self) -> IOResult<()> {
        self.inner.flush().map_err(IOError::from)
    }
    
    /// Seek to a position in the file
    pub fn seek(&mut self, position: u64) -> IOResult<u64> {
        self.inner.seek(SeekFrom::Start(position)).map_err(IOError::from)
    }
    
    /// Seek relative to the current position
    pub fn seek_relative(&mut self, offset: i64) -> IOResult<u64> {
        self.inner.seek(SeekFrom::Current(offset)).map_err(IOError::from)
    }
    
    /// Seek relative to the end of the file
    pub fn seek_from_end(&mut self, offset: i64) -> IOResult<u64> {
        self.inner.seek(SeekFrom::End(offset)).map_err(IOError::from)
    }
    
    /// Get the current position in the file
    pub fn position(&mut self) -> IOResult<u64> {
        self.inner.stream_position().map_err(IOError::from)
    }
}

/// Utility functions for reading and writing files
pub struct FileUtils;

impl FileUtils {
    /// Read the entire contents of a file into a string
    pub fn read_to_string(path: &str) -> IOResult<String> {
        fs::read_to_string(path).map