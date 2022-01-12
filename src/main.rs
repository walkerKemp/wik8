mod emulator;
mod files;

use raylib::prelude::*;
use std::{time, thread};

const FONTSIZE: i32 = 24;

struct RegisterScreenRep<'a> {
  name: &'a str,
  value: u32,
}

fn render_emu_registers(context: &mut RaylibDrawHandle, emu: &mut emulator::Emulator, x: i32, y: i32) {
  let texts: [RegisterScreenRep; 7] = [
    RegisterScreenRep{name: "pcc", value: emu.pcc},
    RegisterScreenRep{name: "acc", value: emu.acc},
    RegisterScreenRep{name: "bak", value: emu.bak},
    RegisterScreenRep{name: "stk", value: emu.stk},
    RegisterScreenRep{name: "lst", value: emu.lst},
    RegisterScreenRep{name: "fl0", value: emu.fl0},
    RegisterScreenRep{name: "fl1", value: emu.fl1},
  ];

  for i in 0..texts.len() {
    context.draw_text(
      format!("{} {}", texts[i].name, texts[i].value).as_str(),
      x, y + FONTSIZE * i as i32, FONTSIZE, Color::WHITE
    );
  }
}

fn render_emu_mem(context: &mut RaylibDrawHandle, emu: &emulator::Emulator, x: i32, y: i32) {
  for i in emu.hsr_range.start..emu.hsr_range.end {
    context.draw_text(
      format!("{} {}", i, emu.mem[i]).as_str(),
      x, y + FONTSIZE * (i - emu.hsr_range.start) as i32, FONTSIZE, Color::WHITE
    );
  }
}

fn main() {
  let file_in = files::get_file_bytes("test.bin");
  let code = files::vu8_to_vu32(&file_in);

  for byte in 0..code.len() {
    println!("byte at {}: {}", byte, code[byte]);
  }

  let mut emu = emulator::Emulator::new(code);
  let mut dt: f32;
  let mut current_time: f32 = 0f32;
  
  let (mut rl, thread) = raylib::init()
    .size(1280, 720)
    .title("Wik-8 Emulator")
    .build();

  while !rl.window_should_close() {
    let mut context = rl.begin_drawing(&thread);
    context.clear_background(Color::BLACK);

    render_emu_registers(&mut context, &mut emu, 32, 16);
    render_emu_mem(&mut context, &emu, 256, 16);

    dt = context.get_frame_time();
    current_time += dt;

    if emu.is_done() {
      emu.running = false;
    }

    if context.is_key_pressed(KeyboardKey::KEY_SPACE) {
      emu.running = !emu.running;
      current_time = 0.016;
    }

    if current_time >= 0.016 {
      if emu.running {
        emu.do_cycle();
        emu.debug_display_regs();
        println!("");
      }
      current_time = 0f32;
    }
  }
}
