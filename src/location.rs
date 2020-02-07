use std::fmt;
use crate::SrcFile;

#[derive(Clone)]
pub struct Location<'a> {
    pub src: &'a str,
    pub path: String,
    pub line: usize,
    pub ch: usize,
}

impl<'a> Location<'a> {
    pub fn new(file: &'a SrcFile, line: usize, ch: usize) -> Location<'a> {
        //! Creates a new Location at the given line and character
        Location {
            src: &file.contents,
            path: file.path.to_string(),
            line,
            ch,
        }
    }

    pub fn start(file: &'a SrcFile) -> Location<'a> {
        //! Creates a new Location starting at the beginning of a file
        Self::new(file, 0, 0)
    }
}

impl<'a> fmt::Debug for Location<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location({}:{}:{})", self.path, self.line, self.ch)
    }
}
