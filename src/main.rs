#![no_std]
#![no_main]
#![allow(dead_code)]

mod io;
pub mod keyboard;
pub mod vga;

use core::panic::PanicInfo;

use crate::vga::VGA;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn crisnet_os_main() -> ! {
    let mut vga: VGA = VGA::new();

    vga.clear_screen();

    vga.println(b"   ____      _                _       ___  ____");
    vga.println(b"  / ___|_ __(_)___ _ __   ___| |_    / _ \\/ ___|");
    vga.println(b" | |   | '__| / __| '_ \\ / _ \\ __|  | | | \\___ \\");
    vga.println(b" | |___| |  | \\__ \\ | | |  __/ |_   | |_| |___) |");
    vga.println(b"  \\____|_|  |_|___/_| |_|\\___|\\__|   \\___/|____/");

    vga.println(b" ");
    vga.println(b" +-----------------------------+");
    vga.println(b" |   Crisnet OS Version 0.3    | ");
    vga.println(b" + ----------------------------+");
    vga.println(b" |    By Crisnet Inc @ 2026    |");
    vga.println(b" +-----------------------------+");
    vga.println(b" |     PRESS ENTER TO REBOOT   |");
    vga.println(b" +-----------------------------+");

    loop {
        let scancode = keyboard::read_scancode();
        if scancode == keyboard::ENTER {
            vga.println(b"Rebooting...");
            io::reboot();
        }
    }
    
}
