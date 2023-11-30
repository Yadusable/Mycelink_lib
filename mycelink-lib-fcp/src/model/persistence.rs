#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Persistence {
    Connection,
    Reboot,
    Forever,
}

impl From<&Persistence> for &str {
    fn from(value: &Persistence) -> Self {
        match value {
            Persistence::Connection => "connection",
            Persistence::Reboot => "reboot",
            Persistence::Forever => "forever",
        }
    }
}

impl From<&Persistence> for Box<str> {
    fn from(value: &Persistence) -> Self {
        Into::<&str>::into(value).into()
    }
}
