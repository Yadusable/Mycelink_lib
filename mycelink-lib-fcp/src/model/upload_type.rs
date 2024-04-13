use crate::model::uri::URI;
use std::path::Path;

pub enum UploadType {
    Direct { data: Box<[u8]> },
    Disk { path: Box<Path> },
    Redirect { target: URI },
}

impl From<&UploadType> for &str {
    fn from(value: &UploadType) -> Self {
        match value {
            UploadType::Direct { .. } => "direct",
            UploadType::Disk { .. } => "disk",
            UploadType::Redirect { .. } => "redirect",
        }
    }
}

impl From<&UploadType> for Box<str> {
    fn from(value: &UploadType) -> Self {
        Into::<&str>::into(value).into()
    }
}
