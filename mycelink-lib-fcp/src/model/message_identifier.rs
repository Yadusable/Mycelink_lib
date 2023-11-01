#[derive(Copy, Clone, Debug)]
pub enum MessageIdentifier {
    ClientHello,
    NodeHello,
}

impl MessageIdentifier {
    pub fn name(&self) -> &'static str {
        match self {
            MessageIdentifier::ClientHello => "ClientHello\n",
            MessageIdentifier::NodeHello => "NodeHello\n",
        }
    }
}
