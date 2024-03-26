use crate::model::contact::Contact;

pub struct MycelinkContact {
    display_name: Box<str>,
}

impl Contact for MycelinkContact {
    fn display_name(&self) -> &str {
        &self.display_name
    }
}
