#[derive(Eq, PartialEq, Debug)]
pub struct Tenant {
    display_name: Box<str>,
}

impl Tenant {
    pub(crate) fn new(display_name: impl Into<Box<str>>) -> Tenant {
        Self {
            display_name: display_name.into(),
        }
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}
