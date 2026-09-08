use crate::io::{KEYBOARD_DATA_PORT, KEYBOARD_STATUS_PORT, inb};

pub const ENTER: u8 = 0x1c;

pub fn read_scancode() -> u8 {
    loop {
        if (inb(KEYBOARD_STATUS_PORT) & 0x01) != 0 {
            break;
        }
    }
    let scan_code = inb(KEYBOARD_DATA_PORT);
    scan_code
}
