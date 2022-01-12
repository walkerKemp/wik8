use std::ops::Range;
pub enum Event {
  Success,
  InvalidDst,
  InvalidStk,
  Failure,
}

pub struct Emulator {
  pub rom: Vec<u32>,
  pub running: bool,
  pub finished: bool,

  pub pcc: u32,
  pub acc: u32,
  pub bak: u32,
  pub stk: u32,
  pub lst: u32,
  pub fl0: u32,
  pub fl1: u32,

  pub mem: [u32; 0x67],
  pub kbi_range: Range<usize>,
  pub hsr_range: Range<usize>,
  pub lsr_range: Range<usize>,
}

impl Emulator {
  pub fn new(rom: Vec<u32>) -> Self {
    Self {
      rom,
      running: false,
      finished: false,

      pcc: 0u32,
      acc: 0u32,
      bak: 0u32,
      stk: 0u32,
      lst: 0u32,
      fl0: 0u32,
      fl1: 0u32,

      mem: [0u32; 0x67],
      kbi_range: 0x00..0x08,
      hsr_range: 0x08..0x28,
      lsr_range: 0x28..0x68,
    }
  }

  pub fn do_cycle(&mut self) -> Event {
    if self.finished {
      return Event::Success;
    }

    let cur_inst = self.next();

    match cur_inst {
      0x10 => { Event::Success },
      0x11 => { 
        let dst = self.next();
        self.lst = *self.get_register(dst);
        *self.get_register(dst) = 0;
        Event::Success
      },
      0x12 => {
        let num = self.next();
        let dst = self.next();
        
        if !self.can_write(dst) {
          self.running = false;
          return Event::InvalidDst;
        }

        self.lst = *self.get_register(dst);
        *self.get_register(dst) = num;
        Event::Success
      },
      0x13 => {
        let dst1 = self.next();
        let dst2 = self.next();

        if !self.can_read(dst1) || !self.can_write(dst2) {
          self.running = false;
          return Event::InvalidDst;
        }

        self.lst = *self.get_register(dst2);
        *self.get_register(dst2) = *self.get_register(dst1);
        Event::Success
      },
      0x14 => {
        let num = self.next();
        self.acc += num;
        self.lst = self.acc;
        Event::Success
      },
      0x15 => {
        let dst = self.next();
        
        if !self.can_read(dst) {
          self.running = false;
          return Event::InvalidDst;
        }

        self.acc += *self.get_register(dst);
        self.lst = self.acc;
        Event::Success
      },
      0x16 => {
        let num = self.next();
        self.acc -= num;
        self.lst = self.acc;
        Event::Success
      },
      0x17 => {
        let dst = self.next();

        if !self.can_read(dst) {
          self.running = false;
          return Event::InvalidDst;
        }

        self.acc -= *self.get_register(dst);
        self.lst = self.acc;
        Event::Success
      },
      0x18 => {
        let temp = self.acc;
        self.acc = self.bak;
        self.bak = temp;
        Event::Success
      },
      0x19 => {
        self.bak = self.acc;
        Event::Success
      },
      0x1A => {
        let off = self.next();
        self.pcc = off;
        Event::Success
      },
      0x1B => {
        let off = self.next();

        if self.acc == 0 {
          self.pcc = off;
        }

        Event::Success
      },
      0x1C => {
        let off = self.next();

        if self.acc != 0 {
          self.pcc = off;
        }

        Event::Success
      },
      0x1D => {
        let off = self.next();

        if self.acc > 0 {
          self.pcc = off;
        }

        Event::Success
      },
      0x1E => {
        let num = self.next();

        if self.acc == num {
          self.fl0 = 1;
        }

        Event::Success
      },
      0x1F => {
        let num = self.next();

        if self.acc != num {
          self.fl0 = 1;
        }

        Event::Success
      },
      0x20 => {
        let num = self.next();

        if self.acc > num {
          self.fl0 = 1;
        }

        Event::Success
      },
      0x21 => {
        let num = self.next();

        if self.acc < num {
          self.fl0 = 1;
        }

        Event::Success
      },
      0x22 => {
        let dst = self.next();
        
        if !self.can_read(dst) {
          self.running = false;
          return Event::InvalidDst;
        }
        
        if self.acc == *self.get_register(dst) {
          self.fl0 = 1;
        }

        Event::Success
      },
      0x23 => {
        let dst = self.next();

        if !self.can_read(dst) {
          self.running = false;
          return Event::InvalidDst;
        }

        if self.acc != *self.get_register(dst) {
          self.fl0 = 1;
        }

        Event::Success
      },
      0x24 => {
        let dst = self.next();

        if !self.can_read(dst) {
          self.running = false;
          return Event::InvalidDst;
        }

        if self.acc > *self.get_register(dst) {
          self.fl0 = 1;
        }

        Event::Success
      },
      0x25 => {
        let dst = self.next();

        if !self.can_read(dst) {
          self.running = false;
          return Event::InvalidDst;
        }

        if self.acc < *self.get_register(dst) {
          self.fl0 = 1;
        }

        Event::Success
      },
      0x26 => {
        // self.mem[self.stk] = self.acc
        if self.kbi_range.contains(&(self.stk as usize)) {
          self.running = false;
          return Event::InvalidStk;
        }

        if !self.lsr_range.contains(&(self.stk as usize)) &&
          !self.hsr_range.contains(&(self.stk as usize)) {
          self.running = false;
          return Event::InvalidStk;
        }

        self.mem[self.stk as usize] = self.acc;
        self.acc = 0;
        Event::Success
      },
      0x27 => {
        if self.kbi_range.contains(&(self.stk as usize)) {
          self.running = false;
          return Event::InvalidStk;
        }

        if !self.lsr_range.contains(&(self.stk as usize)) &&
          !self.hsr_range.contains(&(self.stk as usize)) {
          self.running = false;
          return Event::InvalidStk;
        }

        self.acc = self.mem[self.stk as usize];
        Event::Success
      },
      0x28 => {
        self.stk += 1;
        Event::Success
      },
      0x29 => {
        self.stk -= 1;
        Event::Success
      },
      0x2A => {
        if self.kbi_range.contains(&(self.stk as usize)) {
          self.running = false;
          return Event::InvalidStk;
        }

        if !self.lsr_range.contains(&(self.stk as usize)) &&
          !self.hsr_range.contains(&(self.stk as usize)) {
          self.running = false;
          return Event::InvalidStk;
        }

        self.mem[self.stk as usize] = self.acc;
        self.acc = 0;
        self.stk += 1;
        Event::Success
      },
      0x2B => {
        let off = self.next();
        
        self.fl1 = self.pcc;
        self.pcc = off;

        Event::Success
      },
      0x2C => {
        if self.fl1 > self.rom.len() as u32 {
          self.running = false;
          return Event::Failure;
        }

        self.pcc = self.fl1;
        Event::Success
      }
      _ => panic!("Unreachable")
    }
  }

  pub fn should_close(&mut self) -> bool {
    let ret = self.pcc + 1 > self.rom.len() as u32;
    if ret { self.finished = true; }
    ret
  }

  pub fn next(&mut self) -> u32 {
    let instruction = self.rom[self.pcc as usize];

    if self.should_close() {
      self.running = false;
      return 0;
    }

    self.pcc += 1;
    instruction
  }

  pub fn is_done(&self) -> bool {
    self.pcc as usize > self.rom.len()
  }

  pub fn can_write(&self, op_code: u32) -> bool {
    match op_code {
      0x03 => return true,
      0x04 => return true,
      0x05 => return false,
      0x06 => return true,
      0x07 => return false,
      0x08 => return true,
      0x09 => return true,
      _ => panic!("Invalid register opcode: {}", op_code),
    }
  }

  pub fn can_read(&self, op_code: u32) -> bool {
    match op_code {
      0x03 => return true,
      0x04 => return true,
      0x05 => return false,
      0x06 => return true,
      0x07 => return true,
      0x08 => return true,
      0x09 => return true,
      _ => panic!("Invalid register opcode: {}", op_code),
    }
  }

  pub fn get_register(&mut self, op_code: u32) -> &mut u32 {
    match op_code {
      0x03 => return &mut self.pcc,
      0x04 => return &mut self.acc,
      0x05 => return &mut self.bak,
      0x06 => return &mut self.stk,
      0x07 => return &mut self.lst,
      0x08 => return &mut self.fl0,
      0x09 => return &mut self.fl1,
      _ => panic!("Invalid register opcode: {}", op_code)
    }
  }

  pub fn debug_display_regs(&self) {
    println!(" pcc {}", self.pcc);
    println!(" acc {}", self.acc);
    println!(" bak {}", self.bak);
    println!(" stk {}", self.stk);
    println!(" lst {}", self.lst);
    println!(" fl0 {}", self.fl0);
    println!(" fl1 {}", self.fl1);
  }
}
