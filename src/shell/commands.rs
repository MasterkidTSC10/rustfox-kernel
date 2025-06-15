use crate::graphics::framebuffer::draw_string;
extern crate alloc;
use crate::shell::shell::draw_prompt;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
pub type CommandFn = fn(args: &[&str], fb: &mut [u8], pitch: usize, bpp: usize);

pub struct Command<'a> {
    pub name: &'a str,
    pub func: CommandFn,
}

fn cmd_help(_args: &[&str], fb: &mut [u8], pitch: usize, bpp: usize) {
    draw_string(
        fb,
        pitch,
        bpp,
        10,
        120,
        "Commands: help, clear, echo",
        &[255, 255, 255],
    );
}
pub fn cmd_clear(_args: &[&str], fb: &mut [u8], pitch: usize, bpp: usize) {
    // Clear framebuffer
    for byte in fb.iter_mut() {
        *byte = 0;
    }

    // Redraw prompt
    draw_prompt(fb, pitch, bpp);
}

fn cmd_echo(args: &[&str], fb: &mut [u8], pitch: usize, bpp: usize) {
    let msg = args.join(" ");
    draw_string(fb, pitch, bpp, 10, 120, &msg, &[255, 255, 255]);
}
fn cmd_alloc_test(_args: &[&str], fb: &mut [u8], pitch: usize, bpp: usize) {
    let _boxed = Box::new(42);
    draw_string(fb, pitch, bpp, 10, 140, "Boxed value is 42", &[0, 255, 0]);

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    draw_string(
        fb,
        pitch,
        bpp,
        10,
        160,
        "Vec created with 1, 2, 3",
        &[0, 255, 0],
    );
}
fn cmd_panic(_args: &[&str], fb: &mut [u8], pitch: usize, bpp: usize) {
    panic!("  ");
}

pub fn get_commands<'a>() -> Vec<Command<'a>> {
    vec![
        Command {
            name: "help",
            func: cmd_help,
        },
        Command {
            name: "clear",
            func: cmd_clear,
        },
        Command {
            name: "echo",
            func: cmd_echo,
        },
        Command {
            name: "alloc-test",
            func: cmd_alloc_test,
        },
        Command {
            name: "panic",
            func: cmd_panic,
        },
    ]
}
