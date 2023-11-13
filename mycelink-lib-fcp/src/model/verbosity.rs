type Bitmask = u8;

const SIMPLE_PROGRESS: Bitmask = 1;
const SENDING_TO_NETWORK: Bitmask = 1 << 1;
const COMPATIBILITY_MODE: Bitmask = 1 << 2;
const EXPECTED_HASH: Bitmask = 1 << 3;
// Skip one bit
const EXPECTED_MIME: Bitmask = 1 << 5;
const EXPECTED_DATA_LENGTH: Bitmask = 1 << 6;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Verbosity {
    pub simple_progress: bool,
    pub sending_to_network: bool,
    pub compatibility_mode: bool,
    pub expected_hashes: bool,
    pub expected_mime: bool,
    pub expected_data_length: bool,
}

impl Verbosity {
    pub fn as_bitmask(&self) -> Bitmask {
        let mut res = 0;

        if self.simple_progress {
            res |= SIMPLE_PROGRESS;
        }
        if self.sending_to_network {
            res |= SENDING_TO_NETWORK;
        }
        if self.compatibility_mode {
            res |= COMPATIBILITY_MODE;
        }
        if self.expected_hashes {
            res |= EXPECTED_HASH;
        }
        if self.expected_mime {
            res |= EXPECTED_MIME;
        }
        if self.expected_data_length {
            res |= EXPECTED_DATA_LENGTH;
        }

        res
    }
}
