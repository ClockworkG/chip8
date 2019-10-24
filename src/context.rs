use crate::memory::ROM;

pub trait Context {
    fn new(rom: ROM) -> Self;
    fn run(&mut self);
}
