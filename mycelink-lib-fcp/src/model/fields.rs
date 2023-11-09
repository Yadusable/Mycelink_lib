use crate::decode_error::DecodeError;
use std::slice::Iter;

pub struct Fields {
    fields: Vec<Field>,
}

impl Fields {
    pub fn iter(&self) -> Iter<Field> {
        self.fields.iter()
    }

    pub fn get(&self, key: &str) -> Result<&Field, DecodeError> {
        self.fields
            .iter()
            .find(|e| &*e.key == key)
            .ok_or(DecodeError::MissingField(key.into()))
    }
}

pub struct Field {
    key: Box<str>,
    value: Box<str>,
}

impl Field {
    pub fn new(key: Box<str>, value: Box<str>) -> Self {
        Self { key, value }
    }
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<Vec<Field>> for Fields {
    fn from(value: Vec<Field>) -> Self {
        Fields { fields: value }
    }
}
