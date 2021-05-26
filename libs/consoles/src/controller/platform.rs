use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Platform {
    SegaMegadrive,
    SegaMasterSystem,
    Nes,
    Snes,
}

use std::str::FromStr;

impl FromStr for Platform {
    type Err = String;
    fn from_str(platform: &str) -> Result<Self, Self::Err> {
        match platform {
            "SegaMegadrive" => Ok(Platform::SegaMegadrive),
            "SegaMasterSystem" => Ok(Platform::SegaMasterSystem),
            "NES" => Ok(Platform::Nes),
            "SNES" => Ok(Platform::Snes),
            _ => Err(String::from("Platform not recognised!")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_from_str() {
        let sega_megadrive: Platform = "SegaMegadrive".parse().unwrap();
        let sega_master_system: Platform = "SegaMasterSystem".parse().unwrap();
        let nes: Platform = "NES".parse().unwrap();
        let snes: Platform = "SNES".parse().unwrap();

        assert_eq!(sega_megadrive, Platform::SegaMegadrive);
        assert_eq!(sega_master_system, Platform::SegaMasterSystem);
        assert_eq!(nes, Platform::Nes);
        assert_eq!(snes, Platform::Snes);
    }
}
