use std::fs::{self, DirEntry};
use std::{io, path::Path};

pub struct VisitDir {
    root: Box<dyn Iterator<Item = io::Result<DirEntry>>>,
    children: Box<dyn Iterator<Item = VisitDir>>,
}

impl VisitDir {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let root = Box::new(fs::read_dir(&path)?);
        let children = Box::new(fs::read_dir(&path)?.filter_map(|e| {
            let e = e.ok()?;
            if e.file_type().ok()?.is_dir() {
                return Some(VisitDir::new(e.path()).ok()?);
            }
            None
        }));
        Ok(VisitDir { root, children })
    }

    pub fn entries(self) -> Box<dyn Iterator<Item = io::Result<DirEntry>>> {
        Box::new(
            self.root
                .chain(self.children.map(|s| s.entries()).flatten()),
        )
    }
}

impl Iterator for VisitDir {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.root.next() {
            return Some(item);
        }
        if let Some(child) = self.children.next() {
            self.root = child.entries();
            return self.next();
        }
        None
    }
}
