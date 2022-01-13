mod emulator;
mod files;

use raylib::prelude::*;
use std::{thread, time::Duration};

fn main() {
  let file_in = files::get_file_bytes("test.bin");
  let source_file = files::cast_u8_to_i32(&file_in);

  let mut emu = emulator::Emulator::new(&source_file);

  emu.start();

  while !emu.is_killed {
    emu.print_registers();
    emu.cycle();
    thread::sleep(Duration::from_millis(125));
  }

  emu.print_registers();

  /*
  let (mut rl, thread) = raylib::init()
    .size(1280, 720)
    .title("Wik-8 Emulator")
    .build();

  while !rl.window_should_close() {
    let mut context = rl.begin_drawing(&thread);
    context.clear_background(Color::BLACK);
  }
  */
}
