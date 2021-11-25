use crate::cpu::CPUModifiable;
use crate::common::{Byte, Address};

pub fn ld<Val1: CPUModifiable, Val2: CPUModifiable>(mut a: Val1, b: Val2) {
    a.set16(b.get16());
}

pub fn inc<Val: CPUModifiable>(val: &mut Val) {
    val.set16(val.get16() + 1);
}

pub fn dec<Val: CPUModifiable>(val: &mut Val) {
    val.set16(val.get16() - 1);
}

pub fn bytes_to_dword<Val1: CPUModifiable, Val2: CPUModifiable>(bytes: (Val1, Val2)) -> Address {
    (bytes.0.get8() as Address) << 8 | bytes.1.get8() as Address
}

pub fn dword_to_bytes<Val: CPUModifiable>(dword: Val) -> (Byte, Byte) {
    ((dword.get16() >> 8) as Byte, dword.get16() as Byte)
}