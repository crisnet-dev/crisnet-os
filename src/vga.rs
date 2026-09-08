pub struct VGA {
    cursor_x: u8,
    cursor_y: u8,
    vga_memory: *mut u16,
}

impl VGA {
    const VGA_WIDTH: usize = 80;
    const VGA_HEIGHT: usize = 25;

    pub fn new() -> VGA {
        VGA {
            cursor_x: 0,
            cursor_y: 0,
            vga_memory: 0xb8000 as *mut u16,
        }
    }

    pub fn clear_screen(&self) {
        for i in 0..(80 * 25) {
            unsafe {
                self.vga_memory
                    .add(i)
                    .write_volatile((0x0F << 8) as u16 | ' ' as u16);
            }
        }
    }

    pub fn advance_cursor(&mut self) {
        self.cursor_x += 1;
        if self.cursor_x >= Self::VGA_WIDTH as u8 - 1 {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }
    }

    pub fn jump_line(&mut self) {
        self.cursor_x = 0;
        if self.cursor_y >= Self::VGA_HEIGHT as u8 - 1 {
            self.cursor_y = Self::VGA_HEIGHT as u8 - 1;
        }
        self.cursor_y += 1;
    }

    pub fn print_char(&mut self, c: char) {
        if c == '\n' {
            self.jump_line();
        } else {
            let index = self.cursor_y as usize * Self::VGA_WIDTH + self.cursor_x as usize;
            unsafe {
                self.vga_memory
                    .add(index)
                    .write_volatile((0x0F << 8) as u16 | c as u16);
            }
            self.advance_cursor();
        }
    }

    pub fn print(&mut self, content: &[u8]) {
        for &byte in content {
            self.print_char(byte as char);
        }
    }

    pub fn println(&mut self, content: &[u8]) {
        self.print(content);
        self.jump_line();
    }
}
