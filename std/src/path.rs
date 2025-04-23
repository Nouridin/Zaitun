use std::path::{Path, PathBuf};

pub struct SafePath {
    inner: PathBuf,
}

impl SafePath {
    pub fn new(path: &str) -> Self {
        SafePath {
            inner: PathBuf::from(path),
        }
    }
    
    pub fn join(&self, component: &str) -> Self {
        SafePath {
            inner: self.inner.join(component),
        }
    }
    
    pub fn to_string(&self) -> String {
        self.inner.to_string_lossy().into_owned()
    }
    
    pub fn is_absolute(&self) -> bool {
        self.inner.is_absolute()
    }
    
    pub fn normalize(&self) -> Self {
        // Platform-specific path normalization
        // ... existing code ...
        self.clone()
    }
    
    pub fn platform_separator() -> &'static str {
        if cfg!(windows) {
            "\\"
        } else {
            "/"
        }
    }
}

impl Clone for SafePath {
    fn clone(&self) -> Self {
        SafePath {
            inner: self.inner.clone(),
        }
    }
}