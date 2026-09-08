pub const KEYBOARD_STATUS_PORT: u16 = 0x64;
pub const KEYBOARD_DATA_PORT: u16 = 0x60;

pub fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nostack, nomem)
        );
    }

    value
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nostack, nomem)
        );
    }
}

pub fn reboot() {
    outb(KEYBOARD_STATUS_PORT, 0xFE);
}
