const ORANGE: [u8; 3] = [255, 100, 0];
const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];
const NONE: [u8; 3] = [0, 0, 0];

const FOX_ART: [[&[u8; 3]; 8]; 8] = [
    [
        &NONE, &BLACK, &ORANGE, &ORANGE, &ORANGE, &ORANGE, &BLACK, &NONE,
    ],
    [
        &BLACK, &ORANGE, &WHITE, &ORANGE, &ORANGE, &WHITE, &ORANGE, &BLACK,
    ],
    [
        &ORANGE, &ORANGE, &ORANGE, &ORANGE, &ORANGE, &ORANGE, &ORANGE, &ORANGE,
    ],
    [
        &ORANGE, &BLACK, &ORANGE, &ORANGE, &ORANGE, &ORANGE, &BLACK, &ORANGE,
    ],
    [
        &ORANGE, &WHITE, &BLACK, &ORANGE, &ORANGE, &BLACK, &WHITE, &ORANGE,
    ],
    [
        &ORANGE, &ORANGE, &WHITE, &WHITE, &WHITE, &WHITE, &ORANGE, &ORANGE,
    ],
    [
        &ORANGE, &ORANGE, &ORANGE, &WHITE, &WHITE, &ORANGE, &ORANGE, &ORANGE,
    ],
    [
        &NONE, &ORANGE, &ORANGE, &ORANGE, &ORANGE, &ORANGE, &ORANGE, &NONE,
    ],
];

const PIXEL_SIZE: usize = 4; // Each "fox pixel" is 2×2 screen pixels

fn draw_pixel(fb_mem: &mut [u8], x: usize, y: usize, color: &[u8; 3], pitch: usize, bpp: usize) {
    let offset = y * pitch + x * bpp;
    if offset + 2 < fb_mem.len() {
        fb_mem[offset] = color[2]; // B
        fb_mem[offset + 1] = color[1]; // G
        fb_mem[offset + 2] = color[0]; // R
    }
}

pub fn draw_fox(fb_mem: &mut [u8], pitch: usize, bpp: usize, start_x: usize, start_y: usize) {
    for (y, row) in FOX_ART.iter().enumerate() {
        for (x, &color) in row.iter().enumerate() {
            for dy in 0..PIXEL_SIZE {
                for dx in 0..PIXEL_SIZE {
                    draw_pixel(
                        fb_mem,
                        start_x + x * PIXEL_SIZE + dx,
                        start_y + y * PIXEL_SIZE + dy,
                        color,
                        pitch,
                        bpp,
                    );
                }
            }
        }
    }
}
