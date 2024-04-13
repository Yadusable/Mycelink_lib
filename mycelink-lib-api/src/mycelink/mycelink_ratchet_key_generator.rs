use crate::crypto::key_material::KeyMaterial;
use crate::crypto::ratchet::Ratchet;
use hex::ToHex;
use mycelink_lib_fcp::model::uri::URI;

pub trait MycelinkRatchetKeyGenerator {
    fn generate_send_message_ksk(&self) -> URI;
    fn generate_message_encryption_key(&self) -> KeyMaterial;
}

impl MycelinkRatchetKeyGenerator for Ratchet {
    fn generate_send_message_ksk(&self) -> URI {
        let fcp_upload_key = self.current_key("Mycelink v1 message upload key");
        let fcp_upload_key: Box<str> = fcp_upload_key.encode_hex::<String>().into();
        format!("KSK@Mycelink_v1_channel_message_{fcp_upload_key}")
            .as_str()
            .try_into()
            .unwrap()
    }

    fn generate_message_encryption_key(&self) -> KeyMaterial {
        self.current_key("Mycelink v1 encrypt message")
    }
}
