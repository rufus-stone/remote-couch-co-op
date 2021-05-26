use std::convert::TryFrom;

pub enum SegaMegadriveControls {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    C,
    Select,
    Start,
}

impl From<&SegaMegadriveControls> for u8 {
    fn from(input: &SegaMegadriveControls) -> Self {
        match input {
            SegaMegadriveControls::Up => 0,
            SegaMegadriveControls::Down => 1,
            SegaMegadriveControls::Left => 2,
            SegaMegadriveControls::Right => 3,
            SegaMegadriveControls::A => 4,
            SegaMegadriveControls::B => 5,
            SegaMegadriveControls::C => 6,
            SegaMegadriveControls::Select => 7,
            SegaMegadriveControls::Start => 8,
        }
    }
}

impl TryFrom<u8> for SegaMegadriveControls {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        match input {
            x if x == SegaMegadriveControls::Up as u8 => Ok(SegaMegadriveControls::Up),
            x if x == SegaMegadriveControls::Down as u8 => Ok(SegaMegadriveControls::Down),
            x if x == SegaMegadriveControls::Left as u8 => Ok(SegaMegadriveControls::Left),
            x if x == SegaMegadriveControls::Right as u8 => Ok(SegaMegadriveControls::Right),
            x if x == SegaMegadriveControls::A as u8 => Ok(SegaMegadriveControls::A),
            x if x == SegaMegadriveControls::B as u8 => Ok(SegaMegadriveControls::B),
            x if x == SegaMegadriveControls::C as u8 => Ok(SegaMegadriveControls::C),
            x if x == SegaMegadriveControls::Select as u8 => Ok(SegaMegadriveControls::Select),
            x if x == SegaMegadriveControls::Start as u8 => Ok(SegaMegadriveControls::Start),
            _ => Err("Not a valid Sega Megadrive controller input!".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sega_buttons() {
        let up = u8::from(&SegaMegadriveControls::Up);
        let down = u8::from(&SegaMegadriveControls::Down);
        let left = u8::from(&SegaMegadriveControls::Left);
        let right = u8::from(&SegaMegadriveControls::Right);
        let a = u8::from(&SegaMegadriveControls::A);
        let b = u8::from(&SegaMegadriveControls::B);
        let c = u8::from(&SegaMegadriveControls::C);
        let select = u8::from(&SegaMegadriveControls::Select);
        let start = u8::from(&SegaMegadriveControls::Start);

        assert_eq!(up, 0);
        assert_eq!(down, 1);
        assert_eq!(left, 2);
        assert_eq!(right, 3);
        assert_eq!(a, 4);
        assert_eq!(b, 5);
        assert_eq!(c, 6);
        assert_eq!(select, 7);
        assert_eq!(start, 8);

        //let up: SegaMegadriveControls = up.try_into();
    }
}
