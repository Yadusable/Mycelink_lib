#[derive(Copy, Clone, Debug)]
pub enum MessageIdentifier {
    ClientHello,
}

impl MessageIdentifier {
    pub fn name(&self) -> &'static str {
        match self {
            MessageIdentifier::ClientHello => "ClientHello\n",
        }
    }
}
