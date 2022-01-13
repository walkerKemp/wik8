use std::ops::Range;

const MAXMEM: i32 = 96006;

pub struct Emulator {
  pub is_running: bool,
  pub is_killed: bool,

  pub pcc: i32,
  pub acc: i32,
  pub bak: i32,
  pub stk: i32,
  pub fl0: i32,
  pub fl1: i32,
  pub mem: [i32; MAXMEM as usize],
  pub rom_range: Range<i32>,
  pub kbi_range: Range<i32>,
  pub mos_range: Range<i32>,
  pub ram_range: Range<i32>,
  pub mem_range: Range<i32>,
}


impl Emulator {
  pub fn new(rom: &Vec<i32>) -> Self {
    let mut s = Self {
      is_running: false,
      is_killed: false,
      pcc: 0,
      acc: 0,
      bak: 0,
      stk: 32005,
      fl0: 0,
      fl1: 0,
      mem: [0; MAXMEM as usize],
      rom_range: 0..32000,
      kbi_range: 32000..32002,
      mos_range: 32002..32005,
      ram_range: 32005..96006,
      mem_range: 0..MAXMEM,
    };

    for i in 0..rom.len() {
      if !s.rom_range.contains(&(i as i32)) {
        panic!(" [!] Rom is too big");
      }

      s.mem[i] = rom[i];
    }

    s
  }

  pub fn cycle(&mut self) {
    let inst = self.next();

    match inst {
      0x20 => {},
      0x21 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          *self.get_register(&dst) = 0;
        }
      },
      0x22 => {
        let dst = self.next();
        let num = self.next();
        if self.can_mut(&dst) {
          *self.get_register(&dst) = num;
        }
      }
      0x23 => {
        let dst1 = self.next();
        let dst2 = self.next();
        if self.can_mut(&dst1) && self.can_mut(&dst2) {
          *self.get_register(&dst1) = *self.get_register(&dst2);
        } 
      },
      0x24 => {
        self.acc += self.next();
      },
      0x25 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          self.acc += *self.get_register(&dst);
        }
      },
      0x26 => {
        self.acc -= self.next();
      },
      0x27 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          self.acc -= *self.get_register(&dst);
        }
      },
      0x28 => {
        let temp = self.acc;
        self.acc = self.bak;
        self.bak = temp
      },
      0x29 => {
        self.bak = self.acc;
      },
      0x2a => {
        let off = self.next();
        self.pcc = off;
      },
      0x2b => {
        let off = self.next();
        if self.acc == 0 {
          self.pcc = off;
        }
      },
      0x2c => {
        let off = self.next();
        if self.acc != 0 {
          self.pcc = off;
        }
      },
      0x2d => {
        let off = self.next();
        if self.acc > 0 {
          self.pcc = off;
        }
      },
      0x2e => {
        let off = self.next();
        if self.acc < 0 {
          self.pcc = off;
        }
      },
      0x2f => {
        let num = self.next();
        if self.acc == num {
          self.fl0 |= 0b0000_0000_0000_0001;
        } else {
          self.fl0 &= 0b1111_1111_1111_1110;
        }
      },
      0x30 => {
        let num = self.next();
        if self.acc != num {
          self.fl0 |= 0b0000_0000_0000_0010;
        } else {
          self.fl0 &= 0b1111_1111_1111_1101;
        }
      },
      0x31 => {
        let num = self.next();
        if self.acc > num {
          self.fl0 |= 0b0000_0000_0000_0100;
        } else {
          self.fl0 &= 0b1111_1111_1111_1011;
        }
      },
      0x32 => {
        let num = self.next();
        if self.acc < num {
          self.fl0 |= 0b0000_0000_0000_1000;
        } else {
          self.fl0 &= 0b1111_1111_1111_0111;
        }
      },
      0x33 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          if self.acc == *self.get_register(&dst) {
            self.fl0 |= 0b0000_0000_0001_0000;
          } else {
            self.fl0 &= 0b1111_1111_1110_1111;
          }
        }
      },
      0x34 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          if self.acc != *self.get_register(&dst) {
            self.fl0 |= 0b0000_0000_0010_0000;
          } else {
            self.fl0 &= 0b1111_1111_1101_1111;
          }
        }
      },
      0x35 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          if self.acc > *self.get_register(&dst) {
            self.fl0 |= 0b0000_0000_0100_0000;
          } else {
            self.fl0 &= 0b1111_1111_1011_1111;
          }
        }
      },
      0x36 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          if self.acc < *self.get_register(&dst) {
            self.fl0 |= 0b0000_0000_1000_0000;
          } else {
            self.fl0 &= 0b1111_1111_0111_1111;
          }
        }
      },
      0x37 => {
        let num = self.next();
        if self.ram_range.contains(&self.stk) {
          self.mem[self.stk as usize] = num;
        } else {
          panic!(" [!] Attempted write to invalid or read only memory.");
        }
      }
      0x38 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          if self.ram_range.contains(&self.stk) {
            self.mem[self.stk as usize] = *self.get_register(&dst);
          } else {
            panic!(" [!] Attempted write to invalid or read only memory.");
          }
        }
      },
      0x39 => {
        let dst = self.next();
        if self.can_mut(&dst) {
          if self.mem_range.contains(&self.stk) {
            *self.get_register(&dst) = self.mem[self.stk as usize];
          } else {
            panic!(" [!] Attempted read from invalid memory.");
          }
        }
      },
      0x3a => {
        self.stk += 1;
      },
      0x3b => {
        self.stk -= 1;
      },
      0x3c => {
        let off = self.next();
        self.fl1 = self.pcc;
        self.pcc = off;
      },
      0x3d => {
        self.pcc = self.fl0;
      },
      0x3e | 0x3f | 0x40 | 0x41 => { unimplemented!(" [-] Rasterizer not implemented."); },
      0x42 => {
        self.is_killed = true;
      }
      _ => panic!(" [!] Invalid opcode {}.", inst),
    }
  }

  pub fn start(&mut self) {
    if self.is_running {
      panic!(" [!] Emulator is already running.");
    }

    self.is_running = true;
  }

  pub fn next(&mut self) -> i32 {
    if self.pcc < 0 {
      panic!(" [!] Program counter cant be less than 0.");
    }

    let inst = self.mem[self.pcc as usize];
    self.pcc += 1;
    inst
  }

  pub fn peek(&self, offset: &i32) -> i32  {
    if self.pcc + offset < 0 {
      panic!(" [!] Invalid peek location {}", self.pcc + offset);
    }

    if !self.mem_range.contains(&(self.pcc + offset)){
      panic!(" [!] Invalid peek location {}", self.pcc + offset);
    }

    self.mem[(self.pcc + offset) as usize]
  }

  pub fn can_mut(&self, reg: &i32) -> bool {
    match reg {
      0x10 | 0x11 | 0x13 | 0x14 | 0x15 => return true,
      _ => return false,
    }
  }

  pub fn get_register(&mut self, reg: &i32) -> &mut i32 {
    match reg {
      0x10 => return &mut self.pcc,
      0x11 => return &mut self.acc,
      0x12 => return &mut self.bak,
      0x13 => return &mut self.stk,
      0x14 => return &mut self.fl0,
      0x15 => return &mut self.fl1,
      _ => panic!(" [!] Invalid register {}", reg)
    }
  }

  pub fn print_registers(&self) {
    println!(" pcc {}", self.pcc);
    println!(" acc {}", self.acc);
    println!(" bak {}", self.bak);
    println!(" stk {}", self.stk);
    println!(" fl0 {}", self.fl0);
    println!(" fl1 {}", self.fl1);
    println!("");
  }
}
