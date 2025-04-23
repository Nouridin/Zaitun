use std::path::Path;

pub struct File {
    handle: std::fs::File,
    path: Box<Path>,
}

impl File {
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let path = Path::new(path);
        let handle = std::fs::File::open(path)?;
        Ok(File {
            handle,
            path: path.into(),
        })
    }

    // ... existing code ...
}