use serde::{Deserialize, Serialize};

use crate::controller::platform::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ControlPad {
    pub platform: Platform,
    pub buttons: Vec<u8>,
}

impl ControlPad {
    pub fn buttons(&self) -> &[u8] {
        &self.buttons
    }
}

#[derive(Clone)]
pub struct ControlPadBuilder {
    platform: Platform,
    buttons: Vec<u8>,
}

impl ControlPadBuilder {
    pub fn new(platform: Platform) -> ControlPadBuilder {
        ControlPadBuilder {
            platform,
            buttons: Vec::default(),
        }
    }

    pub fn buttons(&mut self, buttons: Vec<u8>) -> &mut Self {
        let mut new = self;
        new.buttons = buttons;
        new
    }

    pub fn build(&mut self) -> ControlPad {
        ControlPad {
            platform: self.platform.clone(),
            buttons: self.buttons.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_pad_builder() {
        let pad = ControlPadBuilder::new(Platform::SegaMegadrive)
            .buttons(vec![1, 2, 3])
            .build();

        assert_eq!(
            pad,
            ControlPad {
                platform: Platform::SegaMegadrive,
                buttons: vec![1, 2, 3]
            }
        )
    }
}
