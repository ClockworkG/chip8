use crate::cpu::CPU;

pub trait Context<'a> {
    fn with_cpu(cpu: &'a mut CPU) -> Self;
    fn run(&mut self);
}
