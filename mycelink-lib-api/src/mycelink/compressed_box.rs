use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub enum CompressedBox {
    Plain(Box<[u8]>),
}

impl CompressedBox {
    pub fn compress<T: Serialize>(data: &T, _hint: CompressionHint) -> Self {
        let mut encoded = Vec::new();
        ciborium::into_writer(data, &mut encoded).unwrap();

        Self::uncompressed(encoded.into())
    }

    pub fn open<T: for<'de> Deserialize<'de>>(
        self,
    ) -> Result<T, ciborium::de::Error<std::io::Error>> {
        match self {
            CompressedBox::Plain(inner) => ciborium::from_reader(inner.deref()),
        }
    }

    fn uncompressed(data: Box<[u8]>) -> Self {
        Self::Plain(data)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CompressionHint {
    None,
    Fast,
    High,
}

pub trait CompressionHinting {
    fn compression_hint(&self) -> CompressionHint;
}
