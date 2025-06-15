#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

// module imports
mod graphics;
mod keyboard;
mod memory;
mod shell;
// import
use core::panic::PanicInfo;
// use graphics::fox::draw_fox;
use core::alloc::Layout;
use graphics::framebuffer::draw_string;
use keyboard::{read_scancode, scancode_to_ascii};
use limine::request::FramebufferRequest;
use shell::commands::cmd_clear;
use shell::shell::draw_prompt;
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn _start(_multiboot_info_addr: usize) -> ! {
    memory::init_heap();
    if let Some(fb_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = fb_response.framebuffers().next() {
            let pitch = fb.pitch() as usize;
            let bpp = fb.bpp() as usize;
            let fb_size = (fb.width() * fb.height() * (bpp as u64 / 8)) as usize;
            let fb_mem = unsafe { core::slice::from_raw_parts_mut(fb.addr() as *mut u8, fb_size) };

            // Draw initial prompt
            draw_prompt(fb_mem, pitch, bpp);

            // Main input loop
            loop {
                if let Some(scancode) = read_scancode() {
                    shell::shell::handle_input(scancode, fb_mem, pitch, bpp);
                    // let bbp = bpp / 4;
                    // let pit = pitch * 2;
                    // draw_fox(fb_mem, pit, bbp, 100 as usize, 100 as usize);
                }
            }
        }
    }

    loop {}
}
static mut FB_MEM: Option<&'static mut [u8]> = None;
static mut FB_PITCH: usize = 0;
static mut FB_BPP: usize = 0;

// Optional fallback (dummy buffer)
static mut FALLBACK_FB: [u8; 1024] = [0; 1024]; // Small dummy buffer

fn get_fb() -> (&'static mut [u8], usize, usize) {
    unsafe {
        let fb: &mut [u8] = match FB_MEM.as_mut() {
            Some(inner) => *inner, // This keeps the borrow, doesn't move
            None => &mut FALLBACK_FB,
        };
        let pitch = if FB_PITCH != 0 { FB_PITCH } else { 32 };
        let bpp = if FB_BPP != 0 { FB_BPP } else { 3 };
        (fb, pitch, bpp)
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        let (fb, pitch, bpp) = get_fb();
        draw_string(
            fb,
            pitch,
            bpp,
            10,
            200,
            "!!! KERNEL PANIC !!!",
            &[255, 0, 0],
        );
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("allocation error");
}
