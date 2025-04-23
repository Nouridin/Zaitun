use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub struct SafeFile {
    inner: Option<File>,
    path: PathBuf,
}

impl SafeFile {
    pub fn open(path: &str) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        Ok(SafeFile {
            inner: Some(file),
            path: PathBuf::from(path),
        })
    }
    
    pub fn create(path: &str) -> Result<Self, io::Error> {
        let file = File::create(path)?;
        Ok(SafeFile {
            inner: Some(file),
            path: PathBuf::from(path),
        })
    }
    
    pub fn read_to_string(&mut self) -> Result<String, io::Error> {
        let mut content = String::new();
        if let Some(file) = &mut self.inner {
            file.read_to_string(&mut content)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "File not open"));
        }
        Ok(content)
    }
    
    pub fn write(&mut self, content: &str) -> Result<(), io::Error> {
        if let Some(file) = &mut self.inner {
            file.write_all(content.as_bytes())?;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "File not open"));
        }
        Ok(())
    }
}

impl Drop for SafeFile {
    fn drop(&mut self) {
        if self.inner.is_some() {
            self.inner.take(); // Ensure file is closed
        }
    }
}

pub fn platform_path_separator() -> &'static str {
    if cfg!(windows) {
        "\\"
    } else {
        "/"
    }
}