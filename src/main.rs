#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(u8)]
enum Color {
    BLUE = 0x9,
    //GREEN = 0x2,
    WHITE = 0x0F,
}

struct CrisnetOS {
    cursor_x: u8,
    cursor_y: u8,
    vga_memory: *mut u8,
}

impl CrisnetOS {
    const VGA_WIDTH: usize = 80;
    const VGA_HEIGHT: usize = 25;

    fn new() -> CrisnetOS {
        CrisnetOS {
            cursor_x: 0,
            cursor_y: 0,
            vga_memory: 0xb8000 as *mut u8,
        }
    }

    fn clear_screen(&self, color: u8) {
        for i in 0..(Self::VGA_WIDTH * Self::VGA_HEIGHT) {
            unsafe {
                *self.vga_memory.add(i * 2) = b' ';
                *self.vga_memory.add(i * 2 + 1) = color;
            }
        }
    }

    fn advance_cursor(&mut self) {
        self.cursor_x += 1;
        if self.cursor_x >= Self::VGA_WIDTH as u8 {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }
    }

    fn jump_line(&mut self) {
        self.cursor_x = 0;
        if self.cursor_y >= Self::VGA_HEIGHT as u8 {
            self.cursor_y = Self::VGA_HEIGHT as u8 - 1;
        }
        self.cursor_y += 1;
    }

    fn print_char(&mut self, c: char) {
        if c == '\n' {
            self.jump_line();
        } else {
            let index = self.cursor_y as usize * Self::VGA_WIDTH + self.cursor_x as usize;
            unsafe {
                *self.vga_memory.add(index * 2) = c as u8;
                *self.vga_memory.add(index * 2 + 1) =
                    ((Color::BLUE as u8) << 4) | Color::WHITE as u8;
                self.advance_cursor();
            }
        }
    }

    fn print(&mut self, content: &[u8]) {
        for &byte in content {
            self.print_char(byte as char);
        }
    }

    fn println(&mut self, content: &[u8]) {
        self.print(content);
        self.jump_line();
    }
}

#[no_mangle]
pub extern "C" fn crisnet_os_main() -> ! {
    let mut crisnet_os = CrisnetOS::new();

    crisnet_os.clear_screen(((Color::BLUE as u8) << 4) | Color::BLUE as u8);

    crisnet_os.println(b"   ____      _                _       ___  ____");
    crisnet_os.println(b"  / ___|_ __(_)___ _ __   ___| |_    / _ \\/ ___|");
    crisnet_os.println(b" | |   | '__| / __| '_ \\ / _ \\ __|  | | | \\___ \\");
    crisnet_os.println(b" | |___| |  | \\__ \\ | | |  __/ |_   | |_| |___) |");
    crisnet_os.println(b"  \\____|_|  |_|___/_| |_|\\___|\\__|   \\___/|____/");

    crisnet_os.println(b" ");
    crisnet_os.println(b" +-----------------------------+");
    crisnet_os.println(b" +    Crisnet OS Version 0.2   + ");
    crisnet_os.println(b" +-----------------------------+");
    crisnet_os.println(b" +      By Crisnet @ 2026      +");
    crisnet_os.println(b" +-----------------------------+");

    loop {}
}
