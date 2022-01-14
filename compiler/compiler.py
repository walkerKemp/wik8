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
  "nop": ([], 0x00),
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
  "psi": ([TYPES.NUM], 0x37),
  "psh": ([TYPES.DST], 0x38),
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
    self.constants = {}
    self.macros = {}
    self.labels = {}
    self.result = []

  def compile(self):
    self.tokenize()
    self.generate_constants()
    self.generate_macros()
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
    self.max_pointer = len(self.tokens)

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

  def generate_constants(self):
    new_toks = []
    self.pointer = 0
    self.max_pointer = len(self.tokens)

    while self.peek(1) != "\0":
      row, col, inst = self.next()
      if inst == "@const":
        const_name = self.next()
        if not self.is_valid_const(const_name[2]):
          print_err_exit(f"./{self.file_name}:{const_name[0]}:{const_name[1]}: Invalid constant name {const_name[2]}")
        const_value = self.next()
        if not const_value[2].isdigit():
          print_err_exit(f"./{self.file_name}:{const_value[0]}:{const_value[1]}: Invalid constant value, must be type number {const_value[2]}")
        self.constants[const_name[2]] = int(const_value[2])
      else:
        new_toks.append((row, col, inst))

    self.tokens = new_toks
    self.pointer = 0
    self.max_pointer = (len(self.tokens))

  def generate_macros(self):
    new_toks = []
    self.pointer = 0
    self.max_pointer = len(self.tokens)

    while self.peek(1) != "\0":
      x, y, inst = self.next()
      if inst == "@macrodef":
        macro = self.next()

        if not self.is_valid_const(macro[2]):
          print_err_exit(f"./{self.file_name}:{macro[0]}:{macro[1]}: Invalid macro name: {macro[2]}")

        if macro[2] in self.constants.keys() or macro[2] in self.macros.keys():
          print_err_exit(f"./{self.file_name}:{macro[0]}:{macro[2]}: Illegal redefinion of {macro[2]}")

        save_pointer = self.pointer
        end_block = None

        while self.peek(1) != "\0":
          x, y, inst = self.next()

          if inst == "@macroend":
            end_block = (self.pointer, (x, y, inst))
            break

        if end_block == None:
          print(f"./{self.file_name}:{macro[0]}:{macro[1]}: Could not find end block for macro {macro[2]}")

        captured_tokens = []

        for i in range(save_pointer, self.pointer - 1):
          captured_tokens.append(self.tokens[i])

        self.pointer = save_pointer
        self.macros[macro[2]] = captured_tokens
      else:
        if inst != "@macroend":
          new_toks.append((x, y, inst))

    self.tokens = new_toks
    self.pointer = 0
    self.max_pointer = len(self.tokens)

    new_toks = []
    while self.peek(1) != "\0":
      x, y, inst = self.next()
      if inst in self.macros.keys():
        new_toks.extend(self.macros[inst])
      else:
        new_toks.append((x, y, inst))

    self.tokens = new_toks
    self.pointer = 0
    self.max_pointer = len(self.tokens)

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

    if not num.isdigit() and num not in self.constants.keys():
      print_err_exit(f"./{self.file_name}:{x}:{y}: Expected number, found {num}")

    if num in self.constants.keys():
      num = self.constants[num]

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

  @staticmethod
  def is_valid_const(con):
    result = re.findall("\A[a-z|_][a-z0-9|_]*$", con)
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

