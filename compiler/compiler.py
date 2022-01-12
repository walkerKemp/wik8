#!/bin/python3

import re
from os.path import exists
from sys import argv

DST = 0
NUM = 1
OFF = 2

class TYPES:
  DST = 0
  NUM = 1
  OFF = 2

instructions = {
  # inst: (num_args, op_code)
  "nop": ([], 0x10),
  "zer": ([TYPES.DST], 0x11),
  "mvi": ([TYPES.NUM, TYPES.DST], 0x12),
  "mov": ([TYPES.DST, TYPES.DST], 0x13),
  "adi": ([TYPES.NUM], 0x14),
  "add": ([TYPES.DST], 0x15),
  "sbi": ([TYPES.NUM], 0x16),
  "sub": ([TYPES.DST], 0x17),
  "swp": ([], 0x18),
  "sav": ([], 0x19),
  "jmp": ([TYPES.OFF], 0x1A),
  "jez": ([TYPES.OFF], 0x1B),
  "jnz": ([TYPES.OFF], 0x1C),
  "jgz": ([TYPES.OFF], 0x1D),
  "cei": ([TYPES.NUM], 0x1E),
  "cni": ([TYPES.NUM], 0x1F),
  "cgi": ([TYPES.NUM], 0x20),
  "cli": ([TYPES.NUM], 0x21),
  "cet": ([TYPES.DST], 0x22),
  "cnt": ([TYPES.DST], 0x23),
  "cgt": ([TYPES.DST], 0x24),
  "clt": ([TYPES.DST], 0x25),
  "psh": ([], 0x26),
  "pop": ([], 0x27),
  "inc": ([], 0x28),
  "dec": ([], 0x29),
  "pnc": ([], 0x2A),
  "fnc": ([TYPES.OFF], 0x2B),
  "ret": ([], 0x2C),
}

registers = {
  "pcc": 0x03,
  "acc": 0x04,
  "bak": 0x05,
  "stk": 0x06,
  "lst": 0x07,
  "fl0": 0x08,
  "fl1": 0x09,
}

def is_valid_label(lab):
  result = re.findall("\A[a-z][a-z0-9]*:$", lab)
  return len(result) == 1

class Lexer:
  def __init__(self, code):
    self.code = code.split(" ")
    self.pointer = -1
    self.max_pointer = len(self.code) - 1
    self.current_word = ""
    self.labels = {}
    self.result = []

    self.parse()

  def parse(self):
    self.generate_labels()

    while self.peek(1) != "\0":
      self.next()

      if instructions.get(self.current_word, None) != None:
        arg_types, op_code = instructions[self.current_word]
        self.result.append(self.convert_byte_size(op_code))

        for i in arg_types:
          if i == TYPES.DST:
            self.do_dst()
          elif i == TYPES.NUM:
            self.do_num()
          elif i == TYPES.OFF:
            self.do_off()

  def convert_byte_size(self, num):
    res = num.to_bytes(4, "big")
    print(f" [!] Converting {num} to {res}")
    return res

  def generate_labels(self):
    for i, v in enumerate(self.code):
      if is_valid_label(v):
        self.labels[v.replace(":", "")] = i
        self.code[i] = "nop"

  def do_dst(self):
    reg = self.next()
    assert reg != "\0", f"Unexpected end of file at {self.pointer}"
    assert registers.get(reg, None) != None, f"Invalid reg {reg}"
    self.result.append(self.convert_byte_size(registers[reg]))

  def do_num(self):
    num = self.next()
    assert num != "\0", f"Unexpected end of file at {self.pointer}"
    assert num.isdigit(), f"Invalid number {num} as {self.pointer}"
    self.result.append(self.convert_byte_size(int(num)))

  def do_off(self):
    lab = self.next()
    assert self.labels.get(lab, None) != None, f"Invalid label {lab}"
    self.result.append(self.convert_byte_size(self.labels[lab]))

  def next(self):
    if self.pointer + 1 > self.max_pointer:
      self.current_word = "\0"
      return self.current_word

    self.pointer += 1
    self.current_word = self.code[self.pointer]
    return self.current_word

  def peek(self, offset):
    if self.pointer + offset > self.max_pointer:
      return "\0"

    return self.code[self.pointer + offset]

def get_code_file(file_path):
  assert exists(file_path), f" [-] File not found: {file_path}"

  data = ""
  with open(file_path, "r") as f:
    data = f.read()
  data = data.replace("\n", " ")

  return data

def write_bytes_file(file_path, code_bytes):
  with open(file_path, "wb") as f:
    for byte in code_bytes:
      f.write(byte)

def main():
  assert len(argv) == 2, f" [-] Invalid number of arguments, is {len(argv) - 1} should be 1"

  file = argv[1]

  code = get_code_file(file)
  lex = Lexer(code)
  result = lex.result

  output_file = ".".join(file.split(".")[:-1]) + ".bin"
  write_bytes_file(output_file, result)

if __name__ == "__main__":
  main()

