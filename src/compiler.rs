use std::fs::{read, read_to_string};
use regex::Regex;

pub enum Token<'a> {
  Instruction(&'a str),
  Destination(&'a str),
  Label(&'a str),
  Number(&'a str),
  Offset(&'a str),
}

pub fn get_file_bytes(path: &str) -> Vec<u8> {
  read(path).expect("Unable to read file.")
}

pub fn get_file_string(path: &str) -> String {
  read_to_string(path).expect("Unable to read file.")
}

pub struct Lexer<'a> {
  pub source: &'a str,
}
