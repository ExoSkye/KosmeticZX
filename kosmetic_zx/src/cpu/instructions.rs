use super::Cpu;

pub mod instruction_defs;
pub mod meta_instructions;

use crate::common::{Byte, Address};

macro_rules! def_insts {
    ($inst_name: ident, $cpu: ident, $inst: ident, $($name: ident, $opcode: literal)+) => {
        match $inst_name {
            $($opcode => { instruction_defs::$name($cpu, $inst) },)*
            _ => { unimplemented!() }
        }
    };
}

pub fn run(cpu: &mut Cpu, instruction: [Byte; 4]) -> u16 {
    let opcode = instruction[0];

    return def_insts!(opcode, cpu, instruction, nop, 0x0)    
}