use core::slice;
use limine::framebuffer::Framebuffer;
use limine::request::FramebufferRequest;

pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub fn draw_red_square() {
    let fb_response = FRAMEBUFFER_REQUEST
        .get_response()
        .expect("No framebuffer response");

    let fb = fb_response
        .framebuffers()
        .next()
        .expect("No framebuffers found");

    let addr = fb.addr();
    let _width = fb.width();
    let height = fb.height();
    let pitch = fb.pitch();
    let bpp = fb.bpp() / 8;

    let buffer_size = (pitch * height) as usize;
    let fb_mem = unsafe { slice::from_raw_parts_mut(addr as *mut u8, buffer_size) };

    for y in 50..150 {
        for x in 50..150 {
            let offset = (y as usize * pitch as usize + x as usize * bpp as usize) as usize;
            if offset + 3 < fb_mem.len() {
                fb_mem[offset] = 0x00; // B
                fb_mem[offset + 1] = 0x00; // G
                fb_mem[offset + 2] = 0xFF; // R
                if bpp == 4 {
                    fb_mem[offset + 3] = 0xFF; // A
                }
            }
        }
    }
}

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 8;
pub static mut FRAMEBUFFER: Option<Framebuffer> = None;

pub fn draw_char_fake(_ch: char) {
    let fb = unsafe { FRAMEBUFFER.as_mut().unwrap() };
    let width = fb.width() as usize;
    let bpp = fb.bpp() as usize / 8;
    let pitch = fb.pitch() as usize;

    let mem = fb.addr() as *mut u8;

    let x = unsafe { CURSOR_X };
    let y = unsafe { CURSOR_Y };

    for dy in 0..CHAR_HEIGHT {
        for dx in 0..CHAR_WIDTH {
            let offset = (y + dy) * pitch + (x + dx) * bpp;
            unsafe {
                let pixel = mem.add(offset);
                pixel.write_volatile(0x00); // Blue
                pixel.add(1).write_volatile(0x00); // Green
                pixel.add(2).write_volatile(0xFF); // Red
            }
        }
    }

    unsafe {
        CURSOR_X += CHAR_WIDTH;
        if CURSOR_X + CHAR_WIDTH >= width {
            CURSOR_X = 0;
            CURSOR_Y += CHAR_HEIGHT;
        }
    }
}

pub fn draw_pixel(fb: &mut [u8], x: usize, y: usize, color: &[u8; 3], pitch: usize, bpp: usize) {
    let pixel_size = (bpp / 8) as usize;
    let offset = y * pitch + x * pixel_size;

    if offset + 2 < fb.len() {
        fb[offset] = color[0];
        fb[offset + 1] = color[1];
        fb[offset + 2] = color[2];
    }
}

use crate::graphics::font::{FONT, FONT_HEIGHT, FONT_WIDTH};

pub fn draw_char(
    fb: &mut [u8],
    pitch: usize,
    bpp: usize,
    x: usize,
    y: usize,
    c: char,
    color: &[u8; 3],
) {
    let glyph = FONT[c as usize];
    for (row, row_bits) in glyph.iter().enumerate() {
        for col in 0..FONT_WIDTH {
            if row_bits & (1 << (7 - col)) != 0 {
                let px = x + col;
                let py = y + row;
                draw_pixel(fb, px, py, color, pitch, bpp);
            }
        }
    }
}

pub fn draw_string(
    fb: &mut [u8],
    pitch: usize,
    bpp: usize,
    mut x: usize,
    y: usize,
    text: &str,
    color: &[u8; 3],
) {
    for c in text.chars() {
        draw_char(fb, pitch, bpp, x, y, c, color);
        x += 8; // advance x-position (8 is typical font width)
    }
}
