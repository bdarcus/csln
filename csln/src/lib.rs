pub mod style;
pub use style::Style;

pub mod bibliography;
pub use bibliography::InputBibliography;

pub mod citation;

pub trait HasFile {
    fn from_file(path: &str) -> Self;
}
