use std::fs;

pub fn get_file_bytes(file_path: &str) -> Vec<u8> {
  fs::read(file_path).expect("Unable to open file.")
}

pub fn vu8_to_vu32(input: &Vec<u8>) -> Vec<u32> {
  if input.len() % 4 != 0 {
    panic!("Invalid file size, length must be multiple of 4.");
  }

  let mut ret: Vec<u32> = Vec::new();
  let mut pointer: usize = 0;

  while pointer < input.len() - 1 {
    let mut buf: [u8; 4] = [0u8; 4];

    for i in 0usize..4usize {
      buf[i] = input[pointer + i];
    }

    let total = u32::from_be_bytes(buf);
    ret.push(total);
    pointer += 4;
  }

  ret
}
