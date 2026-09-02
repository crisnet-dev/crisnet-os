#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_ADDRESS: usize = 0xb8000;

const WHITE: u8 = 0x0F;
const BLUE: u8 = 0x9;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

fn clear_screen(color: u8) {
    let vga = VGA_ADDRESS as *mut u8;

    for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
        unsafe {
            *vga.add(i * 2) = b' ';
            *vga.add(i * 2 + 1) = color;
        }
    }
}

fn advance_cursor(cursor_x: &mut u8, cursor_y: &mut u8) {
    *cursor_x += 1;
    if *cursor_x >= VGA_WIDTH as u8 {
        *cursor_x = 0;
        *cursor_y += 1;
    }
}

fn jump_line(cursor_x: &mut u8, cursor_y: &mut u8) {
    *cursor_x = 0;
    if *cursor_y >= VGA_HEIGHT as u8 {
        *cursor_y = VGA_HEIGHT as u8 - 1;
    }
    *cursor_y += 1;
}

fn print_char(c: char, cursor_x: &mut u8, cursor_y: &mut u8) {
    let vga = VGA_ADDRESS as *mut u8;

    if c == '\n' {
        jump_line(cursor_x, cursor_y);
    } else {
        let index = *cursor_y as usize * VGA_WIDTH + *cursor_x as usize;
        unsafe {
            *vga.add(index * 2) = c as u8;
            *vga.add(index * 2 + 1) = (BLUE << 4) | WHITE;
            advance_cursor(cursor_x, cursor_y);
        }
    }
}

fn print(content: &[u8], cursor_x: &mut u8, cursor_y: &mut u8) {
    for &byte in content {
        print_char(byte as char, cursor_x, cursor_y);
    }
}

fn println(content: &[u8], cursor_x: &mut u8, cursor_y: &mut u8) {
    print(content, cursor_x, cursor_y);
    jump_line(cursor_x, cursor_y);
}

#[no_mangle]
pub extern "C" fn crisnet_os_main() -> ! {
    let mut cursor_x: u8 = 0;
    let mut cursor_y: u8 = 0;

    clear_screen((BLUE << 4) | BLUE);

    println(b"   ____      _                _       ___  ____", &mut cursor_x, &mut cursor_y);
    println(b"  / ___|_ __(_)___ _ __   ___| |_    / _ \\/ ___|", &mut cursor_x, &mut cursor_y);
    println(b" | |   | '__| / __| '_ \\ / _ \\ __|  | | | \\___ \\", &mut cursor_x, &mut cursor_y);
    println(b" | |___| |  | \\__ \\ | | |  __/ |_   | |_| |___) |", &mut cursor_x, &mut cursor_y);
    println(b"  \\____|_|  |_|___/_| |_|\\___|\\__|   \\___/|____/", &mut cursor_x, &mut cursor_y);

    println(b"", &mut cursor_x, &mut cursor_y);
    println(b" +-----------------------------+",&mut cursor_x, &mut cursor_y);
    println(b" +    Crisnet OS Version 0.2   + ",&mut cursor_x, &mut cursor_y);
    println(b" + ----------------------------+",&mut cursor_x, &mut cursor_y);
    println(b" +     By Crisnet Inc @ 2026   +", &mut cursor_x, &mut cursor_y);
    println(b" +-----------------------------+\n",&mut cursor_x, &mut cursor_y);

    loop {
        core::hint::spin_loop();
    }
}
