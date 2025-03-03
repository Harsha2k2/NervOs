use x86_64::instructions::port::Port;
use crate::{print, println};

pub fn init_keyboard() {
    // Wait for keyboard controller to be ready
    let mut command_port = Port::new(0x64);
    let mut data_port = Port::new(0x60);

    // Clear any pending data
    unsafe {
        while command_port.read() & 1 == 1 {
            data_port.read();
        }
    }

    // Enable keyboard interrupts
    unsafe {
        command_port.write(0xAE_u8); // Enable first PS/2 port
        while command_port.read() & 2 != 0 {} // Wait for controller
        command_port.write(0x20_u8); // Read command byte
        while command_port.read() & 1 == 0 {} // Wait for data
        let mut command_byte: u8 = data_port.read();
        command_byte |= 1; // Enable IRQ
        while command_port.read() & 2 != 0 {} // Wait for controller
        command_port.write(0x60_u8); // Write command byte
        while command_port.read() & 2 != 0 {} // Wait for controller
        data_port.write(command_byte);
    }
    println!("Keyboard initialized");
}

pub fn handle_keypress(scancode: u8) {
    // Print scancode in hex for debugging
    print!("[{:#x}] ", scancode);

    // Check if it's a key release (bit 7 set)
    if scancode & 0x80 != 0 {
        return; // Ignore key releases for now
    }

    // Map scancodes to characters
    match scancode {
        // Numbers
        0x02 => print!("1"),
        0x03 => print!("2"),
        0x04 => print!("3"),
        0x05 => print!("4"),
        0x06 => print!("5"),
        0x07 => print!("6"),
        0x08 => print!("7"),
        0x09 => print!("8"),
        0x0A => print!("9"),
        0x0B => print!("0"),

        // Letters - first row
        0x10 => print!("q"),
        0x11 => print!("w"),
        0x12 => print!("e"),
        0x13 => print!("r"),
        0x14 => print!("t"),
        0x15 => print!("y"),
        0x16 => print!("u"),
        0x17 => print!("i"),
        0x18 => print!("o"),
        0x19 => print!("p"),

        // Letters - second row
        0x1E => print!("a"),
        0x1F => print!("s"),
        0x20 => print!("d"),
        0x21 => print!("f"),
        0x22 => print!("g"),
        0x23 => print!("h"),
        0x24 => print!("j"),
        0x25 => print!("k"),
        0x26 => print!("l"),

        // Letters - third row
        0x2C => print!("z"),
        0x2D => print!("x"),
        0x2E => print!("c"),
        0x2F => print!("v"),
        0x30 => print!("b"),
        0x31 => print!("n"),
        0x32 => print!("m"),

        // Special keys
        0x1C => println!(""), // Enter
        0x39 => print!(" "), // Space
        0x0E => print!("\x08"), // Backspace
        0x01 => println!("[ESC]"), // Escape
        0x3B => println!("[F1]"),
        0x3C => println!("[F2]"),
        0x3D => println!("[F3]"),
        0x3E => println!("[F4]"),
        
        // Print scancode for unmapped keys
        _ => println!("[Unknown key: {:#x}]", scancode),
    }
} 