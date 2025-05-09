use std::fmt;
use byteorder::{ByteOrder, LittleEndian};


#[derive(Debug, Clone, Copy)]
pub struct Payload {
    magic: u8, 
    steer: u16,
    throttle: u16
}

impl Payload {
    pub fn new(steer: u16, throttle: u16) -> Self {
        Self {
            magic: 0xFF,
            steer,
            throttle,
        }
    }

    pub fn to_binary(&self) -> [u8; 5] {
        let mut buf = [0u8; 5];
        buf[0] = self.magic;
        LittleEndian::write_u16(&mut buf[1..3], self.steer);
        LittleEndian::write_u16(&mut buf[3..5], self.throttle);
        buf
    }

    pub fn map_range(value: i16, old_min: i16, old_max: i16, new_min: i16, new_max: i16) -> u16 {
        let value = value as i32;
        let old_min = old_min as i32;
        let old_max = old_max as i32;
        let new_min = new_min as i32;
        let new_max = new_max as i32;

        (((value - old_min) * (new_max - new_min)) / (old_max - old_min) + new_min) as u16
    }
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.steer, self.throttle)
    }
}