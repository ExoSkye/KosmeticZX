use crate::cpu::Cpu;
use crate::common::{Byte, Address};
use crate::cpu::instructions::meta_instructions::*;

pub fn nop(cpu: &mut Cpu, instruction: [Byte; 4]) -> u16 {
    4
}

pub fn ld_bc_imm(cpu: &mut Cpu, instruction: [Byte; 4]) -> u16 {
    cpu.set_BC(bytes_to_dword((instruction[0], instruction[1])));
    10
}