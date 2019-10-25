pub const MEMORY_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const REGISTERS_COUNT: usize = 16;
pub const PROGRAM_BEGIN: usize = 0x200;
pub const MAX_ROM_SIZE: usize = 3584;

pub type Nibble = u8;
pub type Byte = u8;
pub type Instruction = u16;
pub type Address = u16;
pub type Register<T> = T;
