use serde::{Deserialize, Serialize};

use crate::controller::controlpad::*;
use crate::controller::platform::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Payload {
    Ping,
    Echo(String),
    Input(ControlPad),
}

/// Encode the Payload type as a single byte (as long as we don't exceed 255 types)
impl From<&Payload> for u8 {
    fn from(payload: &Payload) -> Self {
        match payload {
            Payload::Ping => 0,
            Payload::Echo(_) => 1,
            Payload::Input(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_payload_to_u8() {
        // Ping
        let payload = Payload::Ping;
        let payload_as_u8 = u8::from(&payload);
        assert_eq!(payload_as_u8, 0);

        // Echo
        let payload = Payload::Echo("Hello".to_owned());
        let payload_as_u8 = u8::from(&payload);
        assert_eq!(payload_as_u8, 1);

        // Input
        let payload = Payload::Input(
            ControlPadBuilder::new(Platform::SegaMegadrive)
                .buttons(vec![1, 2, 3])
                .build(),
        );
        let payload_as_u8 = u8::from(&payload);
        assert_eq!(payload_as_u8, 2);
    }

    #[test]
    fn bincode() {
        // Ping
        let payload = Payload::Ping;
        let encoded: Vec<u8> = bincode::serialize(&payload).unwrap();
        assert_eq!(encoded.len(), 4);

        let decoded: Payload = bincode::deserialize(&encoded[..]).unwrap();
        assert_eq!(payload, decoded);

        // Echo
        let payload = Payload::Echo("Hello".to_owned());
        let encoded: Vec<u8> = bincode::serialize(&payload).unwrap();
        assert_eq!(encoded.len(), 17);

        let decoded: Payload = bincode::deserialize(&encoded[..]).unwrap();
        assert_eq!(payload, decoded);

        // Input
        let payload = Payload::Input(
            ControlPadBuilder::new(Platform::SegaMegadrive)
                .buttons(vec![1, 2, 3])
                .build(),
        );
        let encoded: Vec<u8> = bincode::serialize(&payload).unwrap();
        assert_eq!(encoded.len(), 19);

        let decoded: Payload = bincode::deserialize(&encoded[..]).unwrap();
        assert_eq!(payload, decoded);
    }
}
