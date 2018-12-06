pub fn get_nibble_1 (hextet: u16) -> u8  {
    ((hextet & 0xF000) >> 12) as u8
}

pub fn get_nibble_2 (hextet: u16) -> u8  {
    ((hextet & 0x0F00) >> 8) as u8
}

pub fn get_nibble_3 (hextet: u16) -> u8  {
    ((hextet & 0x00F0) >> 4) as u8
}

pub fn get_nibble_4 (hextet: u16) -> u8 {
    (hextet & 0x000F) as u8
}

pub fn concat_nibbles_2 (nibble1: u8, nibble2: u8) -> u8 {
    ((nibble1 << 4) | nibble2) as u8
}

pub fn concat_nibbles_3 (nibble1: u8, nibble2: u8, nibble3: u8) -> u16 {
    ((concat_nibbles_2(nibble1, nibble2) as u16) << 4) | (nibble3 as u16)
}