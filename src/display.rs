use std::fmt;
use crate::memory::Memory;
use crate::specs::Byte;

const FRAME_HEIGHT: usize = 32;

pub struct FrameBuffer {
    buffer: [u64; FRAME_HEIGHT],
    erased: bool,
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer {
            buffer: [0x0; FRAME_HEIGHT],
            erased: false
        }
    }

    fn write_byte(&mut self, x: usize, y: usize, byte: Byte) {
        let mut x_iter = x;
        let mut mask = 0b10000000;

        for i in 0..8 {
            let bit = (byte & mask) >> (7 - i);
            self.write((x_iter, y), bit != 0);
            mask >>= 1;
            x_iter += 1;
            if x_iter == 64 {
                x_iter = 0;
            }
        }
    }

    pub fn clear_screen(&mut self) {
        for i in 0..FRAME_HEIGHT {
            self.buffer[i] = 0x0;
        }
    }

    pub fn write_bytes(&mut self, pos: <Self as Memory>::Address, bytes: &[Byte]) -> bool {
        let (x, mut y) = pos;

        for byte in bytes {
            self.write_byte(x, y, *byte);
            y += 1;
        }

        self.erased
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
        self.erased = false;

        let (x, y) = addr;
        let tmp = self.buffer[y];
        self.buffer[y] ^= (value as u64) << (63 - x);

        if tmp & self.buffer[y] > 0 {
            self.erased = true;
        }
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
