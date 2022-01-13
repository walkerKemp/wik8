use std::fs;

pub fn get_file_bytes(file_path: &str) -> Vec<u8> {
  fs::read(file_path).expect("Unable to open file.")
}

pub fn cast_u8_to_i32(source: &Vec<u8>) -> Vec<i32> {
  let mut ret: Vec<i32> = Vec::new();

  if source.len() % 4 != 0 {
    panic!(" [!] Length of rom is not valid.");
  }

  let mut buf: [u8; 4] = [0u8; 4];
  let mut pointer: usize = 0;

  while pointer < source.len() {
    buf[0] = source[pointer];
    buf[1] = source[pointer + 1];
    buf[2] = source[pointer + 2];
    buf[3] = source[pointer + 3];

    ret.push(i32::from_be_bytes(buf));

    pointer += 4;
  }

  ret
}

