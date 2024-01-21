pub struct Contact {
    identifier: ContactIdentifier,
    display_name: Box<str>,
}

pub struct ContactIdentifier {
    pub_key: Box<str>,
}
