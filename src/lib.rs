pub mod ast;
pub mod error;
pub mod interpreter;
pub mod location;
pub mod parser;
pub mod test;

pub use location::Location;

pub struct SrcFile {
    path: String,
    contents: String,
}
