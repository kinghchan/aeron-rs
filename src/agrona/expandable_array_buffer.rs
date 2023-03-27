use std::mem;
use std::ptr;
use std::cmp::min;
use std::slice;
//use byteorder::{ByteOrder, NativeEndian};

pub const MAX_ARRAY_LENGTH: usize = std::isize::MAX as usize - 8;
pub const INITIAL_CAPACITY: usize = 128;

pub struct ExpandableArrayBuffer {
    byte_array: Vec<u8>,
}

impl ExpandableArrayBuffer {
    pub fn new() -> Self {
        Self::with_capacity(INITIAL_CAPACITY)
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            byte_array: vec![0; initial_capacity],
        }
    }

    pub fn capacity(&self) -> usize {
        self.byte_array.len()
    }

    pub fn is_expandable(&self) -> bool {
        true
    }

    fn ensure_capacity(&mut self, index: i32, length: i32) {
        if index < 0 || length < 0 {
            panic!("negative value: index={} length={}", index, length);
        }

        let resulting_position = index as i64 + length as i64;
        let current_array_length = self.byte_array.len() as i64;
        if resulting_position > current_array_length {
            if resulting_position > std::i32::MAX as i64 {
                panic!("index={} length={} max_capacity={}",
                       index, length, std::i32::MAX);
            }

            let new_capacity = self.calculate_expansion(current_array_length as i32, resulting_position) as usize;
            self.byte_array.resize(new_capacity, 0);
        }
    }

    fn calculate_expansion(&mut self, current_length: i32, required_length: i64) -> i32 {
        let mut value = current_length as i64;
        value = std::cmp::max(value, INITIAL_CAPACITY as i64);

        while value < required_length {
            value = value + (value >> 1);

            if value > MAX_ARRAY_LENGTH as i64 {
                value = MAX_ARRAY_LENGTH as i64;
            }
        }

        value as i32
    }


    pub fn set_memory(&mut self, index: usize, length: usize, value: u8) {
        self.ensure_capacity(index as i32, length as i32);
        self.byte_array[index..index + length].fill(value);
    }
}

#[test]
fn array_expands_correctly() {

}
