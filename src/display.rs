use std::fmt;
use crate::memory::Memory;
use crate::specs::{Address, Byte};

const FRAME_HEIGHT: usize = 32;

pub struct FrameBuffer {
    buffer: [u64; FRAME_HEIGHT]
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer {
            buffer: [0x0; FRAME_HEIGHT]
        }
    }

    fn write_byte(&mut self, x: &mut usize, y: &mut usize, byte: Byte) {
        let mut mask = 0b10000000;
        for i in 0..8 {
            let bit = (byte & mask) >> (7 - i);
            self.write((*x, *y), bit != 0);
            mask >>= 1;
            *x += 1;
            if *x == 64 {
                *x = 0;
                *y += 1;
            }
        }
    }

    pub fn write_bytes(&mut self, pos: <Self as Memory>::Address, bytes: &[Byte]) {
        let (mut x, mut y) = pos;

        for byte in bytes {
            self.write_byte(&mut x, &mut y, *byte);
        }
    }
}

impl Memory for FrameBuffer {
    type Address = (usize, usize);
    type Value = bool;

    fn read(&self, addr: Self::Address) -> Self::Value {
        let (x, y) = addr;
        (self.buffer[y] & (1 << (63 - x))) >> (63 - x) != 0
    }

    fn write(&mut self, addr: Self::Address, value: Self::Value) {
        let (x, y) = addr;
        self.buffer[y] ^= (value as u64) << (63 - x);
    }
}

impl fmt::Display for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X>67}", "\n")?;
        for row in 0..32 {
            write!(f, "X")?;
            for col in 0..64 {
                let chr = if self.read((col, row)) {
                    "▉"
                } else {
                    " "
                };
                write!(f, "{}", chr)?;
            }
            write!(f, "X\n")?;
        }
        write!(f, "{:X>67}", "\n")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let mut buff = FrameBuffer::new();
        buff.buffer[3] |= 0xF0F0F0F0F0F0F0F0;
        assert_eq!(buff.read((3, 3)), true);
        assert_eq!(buff.read((3, 9)), false);
    }
}
