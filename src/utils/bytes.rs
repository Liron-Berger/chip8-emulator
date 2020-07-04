pub fn get_u8_lsb(value: u8) -> u8 {
    value & 1
}

pub fn get_u8_msb(value: u8) -> u8 {
    (value & 0b10000000) >> 7
}