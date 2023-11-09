#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConnectionIdentifier(Box<str>);

impl From<&str> for ConnectionIdentifier {
    fn from(value: &str) -> Self {
        ConnectionIdentifier(value.into())
    }
}
