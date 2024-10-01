// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[inline]
pub fn latin1_to_utf8(char: u8) -> ([u8; 2], usize) {
    if char < 0x80 {
        ([char, 0], 1)
    } else {
        let item = [
            (char >> 6) | 0b1100_0000,
            (char & 0b0011_1111) | 0b1000_0000,
        ];
        (item, 2)
    }
}
