#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(log_syntax)]
extern crate alloc;

// module imports
mod graphics;
mod keyboard;
mod memory;
mod shell;
// import
// use graphics::fox::draw_fox;
use core::alloc::Layout;
use keyboard::{read_scancode, scancode_to_ascii};
use limine::framebuffer::Framebuffer;
use limine::request::FramebufferRequest;
use shell::shell::draw_prompt;
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
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

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("allocation error!");
}
