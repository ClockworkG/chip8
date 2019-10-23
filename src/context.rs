use crate::cpu::CPU;

pub trait Context<'a> {
    fn with_cpu(cpu: &'a CPU) -> Self;
    fn run(&mut self);
}
