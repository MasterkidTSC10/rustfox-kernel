use crate::graphics::font::FONT_WIDTH;
use crate::graphics::framebuffer::{draw_char, draw_string};
use crate::keyboard::scancode_to_ascii;
extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

static SHIFT_PRESSED: AtomicBool = AtomicBool::new(false);

const MAX_INPUT: usize = 2560;
static mut INPUT_BUFFER: [u8; MAX_INPUT] = [0; MAX_INPUT];
static mut INPUT_LEN: usize = 0; // number of valid chars in input buffer
static mut CURSOR_POS: usize = 0; // cursor position within input (0..=INPUT_LEN)
static mut PREV_WAS_E0: bool = false; // for extended scancodes

const INPUT_START_X: usize = 68;
const INPUT_START_Y: usize = 20;
const FONT_HEIGHT: usize = 8; // define your actual font height
static mut INPUT_INDEX: usize = 0;

const PROMPT: &str = "rustos> ";
const PROMPT_X: usize = 10;
const INPUT_Y: usize = 20;
const PROMPT_COLOR: [u8; 3] = [0, 100, 255];
const INPUT_COLOR: [u8; 3] = [255, 255, 255];
const CURSOR_COLOR: [u8; 3] = [255, 255, 255];
const BG_COLOR: [u8; 3] = [0, 0, 0]; // Black background

fn clear_char(fb: &mut [u8], x: usize, y: usize, pitch: usize, bpp: usize) {
    for dy in 0..FONT_HEIGHT {
        for dx in 0..FONT_WIDTH {
            let offset = (y + dy) * pitch + (x + dx) * bpp / 8;
            if offset + 2 < fb.len() {
                fb[offset] = 0;
                fb[offset + 1] = 0;
                fb[offset + 2] = 0;
            }
        }
    }
}

fn redraw_input_line(fb: &mut [u8], pitch: usize, bpp: usize) {
    unsafe {
        // Clear whole input line area first
        let line_width = MAX_INPUT * FONT_WIDTH;
        for dx in 0..line_width {
            for dy in 0..FONT_HEIGHT {
                let offset = (INPUT_START_Y + dy) * pitch + (INPUT_START_X + dx) * bpp / 8;
                if offset + 2 < fb.len() {
                    fb[offset] = BG_COLOR[2];
                    fb[offset + 1] = BG_COLOR[1];
                    fb[offset + 2] = BG_COLOR[0];
                }
            }
        }

        // Draw the prompt again in case it got overwritten
        draw_string(fb, pitch, bpp, PROMPT_X, INPUT_Y, PROMPT, &PROMPT_COLOR);

        // Draw all chars of the input buffer
        for i in 0..INPUT_LEN {
            let x = INPUT_START_X + i * FONT_WIDTH;
            let byte_array = [INPUT_BUFFER[i]];
            let s = core::str::from_utf8_unchecked(&byte_array);
            draw_string(fb, pitch, bpp, x, INPUT_START_Y, s, &INPUT_COLOR);
        }

        // Draw the cursor as a filled rectangle (block cursor)
        let cursor_x = INPUT_START_X + CURSOR_POS * FONT_WIDTH;
        for dy in 0..FONT_HEIGHT {
            for dx in 0..FONT_WIDTH {
                let offset = (INPUT_START_Y + dy) * pitch + (cursor_x + dx) * bpp / 8;
                if offset + 2 < fb.len() {
                    fb[offset] = CURSOR_COLOR[2];
                    fb[offset + 1] = CURSOR_COLOR[1];
                    fb[offset + 2] = CURSOR_COLOR[0];
                }
            }
        }
    }
}

pub fn handle_input(scancode: u8, fb: &mut [u8], pitch: usize, bpp: usize) {
    match scancode {
        0x2A | 0x36 => {
            // Left or Right Shift pressed
            SHIFT_PRESSED.store(true, Ordering::SeqCst);
            return; // No further processing
        }
        0xAA | 0xB6 => {
            // Left or Right Shift released
            SHIFT_PRESSED.store(false, Ordering::SeqCst);
            return; // No further processing
        }
        _ => {}
    }

    // Now process the scancode to ASCII as usual
    let shift = SHIFT_PRESSED.load(Ordering::SeqCst);
    if let Some(ascii) = scancode_to_ascii(scancode, shift) {
        let ascii_byte = ascii as u8;
        unsafe {
            if ascii == b'\n' || ascii == b'\r' {
                let command = core::str::from_utf8_unchecked(&INPUT_BUFFER[..INPUT_INDEX]);

                // Echo the command below the input line
                draw_string(
                    fb,
                    pitch,
                    bpp,
                    PROMPT_X,
                    INPUT_Y + 20,
                    PROMPT,
                    &PROMPT_COLOR,
                );
                draw_string(
                    fb,
                    pitch,
                    bpp,
                    PROMPT_X + PROMPT.len() * FONT_WIDTH,
                    INPUT_Y + 20,
                    command,
                    &INPUT_COLOR,
                );

                // Execute the command
                execute_command(command, fb, pitch, bpp);

                // Clear input buffer and line visually
                for i in 0..INPUT_INDEX {
                    let x = INPUT_START_X + i * FONT_WIDTH;
                    clear_char(fb, x, INPUT_Y, pitch, bpp);
                }

                INPUT_INDEX = 0;
                INPUT_BUFFER = [0; MAX_INPUT];
            } else if ascii == 8 {
                // Backspace
                if INPUT_INDEX > 0 {
                    INPUT_INDEX -= 1;
                    INPUT_BUFFER[INPUT_INDEX] = 0;

                    let x = INPUT_START_X + INPUT_INDEX * FONT_WIDTH;
                    clear_char(fb, x, INPUT_Y, pitch, bpp);
                }
            } else if INPUT_INDEX < MAX_INPUT {
                let x = INPUT_START_X + INPUT_INDEX * FONT_WIDTH;
                INPUT_BUFFER[INPUT_INDEX] = ascii_byte;

                let s = core::str::from_utf8_unchecked(&INPUT_BUFFER[INPUT_INDEX..INPUT_INDEX + 1]);

                draw_string(fb, pitch, bpp, x, INPUT_Y, s, &INPUT_COLOR);

                INPUT_INDEX += 1;
            }
        }
    }
}

pub fn draw_prompt(fb: &mut [u8], pitch: usize, bpp: usize) {
    draw_string(fb, pitch, bpp, PROMPT_X, INPUT_Y, PROMPT, &PROMPT_COLOR);
}

use crate::shell::commands::get_commands;

fn execute_command(command: &str, fb: &mut [u8], pitch: usize, bpp: usize) {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let cmd_name = parts[0];
    let args = &parts[1..];

    for cmd in get_commands() {
        if cmd.name == cmd_name {
            (cmd.func)(args, fb, pitch, bpp);
            return;
        }
    }

    draw_string(fb, pitch, bpp, 10, 120, "Unknown command", &[255, 0, 0]);
}
