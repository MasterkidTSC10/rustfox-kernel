use core::sync::atomic::{AtomicBool, Ordering};
use x86_64::instructions::port::Port;

pub static SHIFT_PRESSED: AtomicBool = AtomicBool::new(false);

const LEFT_SHIFT_DOWN: u8 = 0x2A;
const RIGHT_SHIFT_DOWN: u8 = 0x36;
const LEFT_SHIFT_UP: u8 = 0xAA;
const RIGHT_SHIFT_UP: u8 = 0xB6;

pub fn read_scancode() -> Option<u8> {
    let mut status_port: Port<u8> = Port::new(0x64);
    let mut data_port: Port<u8> = Port::new(0x60);

    let status = unsafe { status_port.read() };
    if status & 1 != 0 {
        let scancode = unsafe { data_port.read() };
        Some(scancode)
    } else {
        None
    }
}

/// Translate scancode into ASCII character, considering shift state.
pub fn scancode_to_ascii(scancode: u8, shift: bool) -> Option<u8> {
    match (scancode, shift) {
        // Row 1
        (0x02, false) => Some(b'1'),
        (0x02, true) => Some(b'!'),
        (0x03, false) => Some(b'2'),
        (0x03, true) => Some(b'"'),
        (0x04, false) => Some(b'3'),
        (0x04, true) => Some(b'?'),
        (0x05, false) => Some(b'4'),
        (0x05, true) => Some(b'$'),
        (0x06, false) => Some(b'5'),
        (0x06, true) => Some(b'%'),
        (0x07, false) => Some(b'6'),
        (0x07, true) => Some(b'&'),
        (0x08, false) => Some(b'7'),
        (0x08, true) => Some(b'/'),
        (0x09, false) => Some(b'8'),
        (0x09, true) => Some(b'('),
        (0x0A, false) => Some(b'9'),
        (0x0A, true) => Some(b')'),
        (0x0B, false) => Some(b'0'),
        (0x0B, true) => Some(b'='),
        (0x0C, false) => Some(b'?'),
        (0x0C, true) => Some(b'`'),
        (0x0D, false) => Some(b'?'),
        (0x0D, true) => Some(b'?'),
        (0x2B, false) => Some(b'\\'),
        (0x2B, true) => Some(b'|'),

        // Row 2
        (0x10, false) => Some(b'q'),
        (0x10, true) => Some(b'Q'),
        (0x11, false) => Some(b'w'),
        (0x11, true) => Some(b'W'),
        (0x12, false) => Some(b'e'),
        (0x12, true) => Some(b'E'),
        (0x13, false) => Some(b'r'),
        (0x13, true) => Some(b'R'),
        (0x14, false) => Some(b't'),
        (0x14, true) => Some(b'T'),
        (0x15, false) => Some(b'z'),
        (0x15, true) => Some(b'Z'), // swapped
        (0x16, false) => Some(b'u'),
        (0x16, true) => Some(b'U'),
        (0x17, false) => Some(b'i'),
        (0x17, true) => Some(b'I'),
        (0x18, false) => Some(b'o'),
        (0x18, true) => Some(b'O'),
        (0x19, false) => Some(b'p'),
        (0x19, true) => Some(b'P'),
        (0x1A, false) => Some(b'?'),
        (0x1A, true) => Some(b'?'),
        (0x1B, false) => Some(b'+'),
        (0x1B, true) => Some(b'*'),

        // Row 3
        (0x1E, false) => Some(b'a'),
        (0x1E, true) => Some(b'A'),
        (0x1F, false) => Some(b's'),
        (0x1F, true) => Some(b'S'),
        (0x20, false) => Some(b'd'),
        (0x20, true) => Some(b'D'),
        (0x21, false) => Some(b'f'),
        (0x21, true) => Some(b'F'),
        (0x22, false) => Some(b'g'),
        (0x22, true) => Some(b'G'),
        (0x23, false) => Some(b'h'),
        (0x23, true) => Some(b'H'),
        (0x24, false) => Some(b'j'),
        (0x24, true) => Some(b'J'),
        (0x25, false) => Some(b'k'),
        (0x25, true) => Some(b'K'),
        (0x26, false) => Some(b'l'),
        (0x26, true) => Some(b'L'),
        (0x27, false) => Some(b'?'),
        (0x27, true) => Some(b'?'),
        (0x28, false) => Some(b'?'),
        (0x28, true) => Some(b'?'),
        (0x29, false) => Some(b'#'),
        (0x29, true) => Some(b'\''),

        // Row 4
        (0x2C, false) => Some(b'y'),
        (0x2C, true) => Some(b'Y'), // swapped
        (0x2D, false) => Some(b'x'),
        (0x2D, true) => Some(b'X'),
        (0x2E, false) => Some(b'c'),
        (0x2E, true) => Some(b'C'),
        (0x2F, false) => Some(b'v'),
        (0x2F, true) => Some(b'V'),
        (0x30, false) => Some(b'b'),
        (0x30, true) => Some(b'B'),
        (0x31, false) => Some(b'n'),
        (0x31, true) => Some(b'N'),
        (0x32, false) => Some(b'm'),
        (0x32, true) => Some(b'M'),
        (0x33, false) => Some(b','),
        (0x33, true) => Some(b';'),
        (0x34, false) => Some(b'.'),
        (0x34, true) => Some(b':'),
        (0x35, false) => Some(b'-'),
        (0x35, true) => Some(b'_'),

        // Space, Enter, Backspace
        (0x39, _) => Some(b' '),
        (0x1C, _) => Some(b'\n'),
        (0x0E, _) => Some(8), // Backspace

        _ => None,
    }
}
