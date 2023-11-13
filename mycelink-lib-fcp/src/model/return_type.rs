use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReturnType {
    Direct,
    Disk { path: Box<Path> },
    None,
}

impl From<&ReturnType> for &str {
    fn from(value: &ReturnType) -> Self {
        match value {
            ReturnType::Direct => "direct",
            ReturnType::Disk { .. } => "disk",
            ReturnType::None => "none",
        }
    }
}
