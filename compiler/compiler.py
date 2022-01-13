#!/bin/python3

import re
from os.path import exists
from sys import argv
from os import _exit
from time import time

class TYPES:
  DST = 0
  NUM = 1
  OFF = 2

instructions = {
  # inst: ([arg_list], op_code)
  "nop": ([], 0x20),
  "zer": ([TYPES.DST], 0x21),
  "mvi": ([TYPES.DST, TYPES.NUM], 0x22),
  "mov": ([TYPES.DST, TYPES.DST], 0x23),
  "adi": ([TYPES.NUM], 0x24),
  "add": ([TYPES.DST], 0x25),
  "sbi": ([TYPES.NUM], 0x26),
  "sub": ([TYPES.DST], 0x27),
  "swp": ([], 0x28),
  "sav": ([], 0x29),
  "jmp": ([TYPES.OFF], 0x2a),
  "jez": ([TYPES.OFF], 0x2b),
  "jnz": ([TYPES.OFF], 0x2c),
  "jgz": ([TYPES.OFF], 0x2d),
  "jlz": ([TYPES.OFF], 0x2e),
  "cei": ([TYPES.NUM], 0x2f),
  "cni": ([TYPES.NUM], 0x30),
  "cgi": ([TYPES.NUM], 0x31),
  "cli": ([TYPES.NUM], 0x32),
  "cet": ([TYPES.DST], 0x33),
  "cnt": ([TYPES.DST], 0x34),
  "cgt": ([TYPES.DST], 0x35),
  "clt": ([TYPES.DST], 0x36),
  "psh": ([TYPES.DST], 0x37),
  "psi": ([TYPES.NUM], 0x38),
  "pop": ([TYPES.DST], 0x39),
  "inc": ([], 0x3a),
  "dec": ([], 0x3b),
  "fnc": ([TYPES.OFF], 0x3c),
  "ret": ([], 0x3d),
  "rsi": ([TYPES.NUM, TYPES.NUM], 0x3e),
  "rsd": ([TYPES.NUM, TYPES.DST], 0x3f),
  "rai": ([TYPES.DST, TYPES.NUM], 0x40),
  "rad": ([TYPES.DST, TYPES.DST], 0x41),
  "kil": ([], 0x42)
}

registers = {
  "pcc": 0x10,
  "acc": 0x11,
  "bak": 0x12,
  "stk": 0x13,
  "fl0": 0x14,
  "fl1": 0x15,
}

def print_err_exit(message):
  print(message)
  _exit(1)

def print_warn_exit(message):
  print(message)

class Lexer:
  def __init__(self, file_name, code):
    self.file_name = file_name
    self.code = code
    self.tokens = []
    self.pointer = 0
    self.max_pointer = 0
    self.labels = {}
    self.result = []

  def compile(self):
    self.tokenize()
    self.generate_labels()
    self.parse()
    return self.result

  def tokenize(self):
    for i, line in enumerate(self.code.split("\n")):
      pointer_in_line = 0
      while pointer_in_line < len(line):
        word = ""
        while line[pointer_in_line].isspace():
          pointer_in_line += 1
        while pointer_in_line < len(line) and not line[pointer_in_line].isspace():
          word += line[pointer_in_line]
          pointer_in_line += 1
        self.tokens.append((i, pointer_in_line - len(word), word))
    self.max_pointer = len(self.tokens) - 1

  def parse(self):
    while self.peek(1) != "\0":
      x, y, tok = self.next()

      if tok in instructions.keys():
        args, op_code = instructions[tok]

        self.result.append(op_code.to_bytes(4, "big"))

        for i in args:
          if i == TYPES.NUM:
            self.do_num()
          if i == TYPES.DST:
            self.do_dst()
          if i == TYPES.OFF:
            self.do_off()
      elif self.is_valid_label(tok):
        pass
      else:
        print_err_exit(f"./{self.file_name}:{x}:{y}: Unexpected token {tok}")

  def next(self):
    if self.pointer > self.max_pointer:
      return "\0"

    token = self.tokens[self.pointer]
    self.pointer += 1
    return token

  def peek(self, offset):
    if self.pointer + offset > self.max_pointer:
      return "\0"

    token = self.tokens[self.pointer]
    return token

  def do_num(self):
    tok = self.next()
    if tok == "\0":
      print_err_exit(f"./{self.file_name}: Expected number, found EOF")

    x, y, num = tok

    if not num.isdigit():
      print_err_exit(f"./{self.file_name}:{x}:{y}: Expected number, found {num}")

    if int(num) > 2147483647:
      print_err_exit(f"./{self.file_name}:{x}:{y}: Number {num} is too big, must be within i32")

    if int(num) < -2147483648:
      print_err_exit(f"./{self.file_name}:{x}:{y}: Number {num} is too small, must be within i32")

    self.result.append(int(num).to_bytes(4, "big"))

  def do_dst(self):
    tok = self.next()
    if tok == "\0":
      print_err_exit(f"./{self.file_name}: Expected register, found EOF")

    x, y, dst = tok

    if dst not in registers.keys():
      print_err_exit(f"./{self.file_name}:{x}:{y}: Expected register, found {dst}")

    self.result.append(registers[dst].to_bytes(4, "big"))

  def do_off(self):
    tok = self.next()
    if tok == "\0":
      print_err_exit(f"./{self.file_name}: Expected label, found EOF")

    x, y, lab = tok

    if lab not in self.labels.keys():
      print_err_exit(f"./{self.file_name}:{x}:{y}: Unknown label: {lab}")

    self.result.append(self.labels[lab].to_bytes(4, "big"))

  def generate_labels(self):
    for i, tok in enumerate(self.tokens):
      if self.is_valid_label(tok[2]):
        if tok[2].replace(":", "") in self.labels.keys():
          print_warn_exit(f"./{self.file_name}:{x}:{y}: Warning: Redefinition of label{tok[1]}")

        self.labels[tok[2].replace(":", "")] = i
        self.tokens[i] = (tok[0], tok[1], "nop")

  @staticmethod
  def is_valid_label(lab):
    result = re.findall("\A[a-z][a-z0-9]*:$", lab)
    return len(result) == 1

def main():
  _start_time = time()

  if len(argv) != 2:
    print("Invalid number of arguments")
    print_err_exit(f"USAGE: ./{__file__} <file_path>")

  file = argv[1]
  if not exists(file):
    print_err_exit(f"File not found: {file}")

  data = ""
  with open(file, "r") as f:
    data = f.read()

  lex = Lexer(file, data)
  lex.compile()

  out_file = f"{''.join(file.split('.')[:-1])}.bin"
  with open(out_file, "wb") as f:
    for byte in lex.result:
      f.write(byte)

  print(f"Compilation successful {round(time() - _start_time, 5)}s")

if __name__ == "__main__":
  main()

