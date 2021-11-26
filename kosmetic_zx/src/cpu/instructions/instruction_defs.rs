use crate::cpu::Cpu;
use crate::common::{Byte, Address};
use crate::cpu::instructions::meta_instructions::*;

pub fn nop(cpu: &mut Cpu, instruction: [Byte; 4]) -> (u16, u8) {
    (4, 1)
}

pub fn ld_bc_imm(cpu: &mut Cpu, instruction: [Byte; 4]) -> (u16, u8) {
    cpu.set_BC(bytes_to_dword((instruction[1], instruction[2])));
    (10, 3)
}

pub fn ld_bc_a(cpu: &mut Cpu, instruction: [Byte; 4]) -> (u16, u8) {
    cpu.set_BC(bytes_to_dword((cpu.A, 0)));
    (7, 1)
}

pub fn inc_bc(cpu: &mut Cpu, instruction: [Byte; 4]) -> (u16, u8) {
    cpu.set_BC(cpu.BC() + 1);
    (6, 1)
}

pub fn inc_b(cpu: &mut Cpu, instruction: [Byte; 4])  -> (u16, u8) {
    inc(cpu.BC());
    (4, 1)
}