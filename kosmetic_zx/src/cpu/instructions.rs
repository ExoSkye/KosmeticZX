use crate::cpu::CPUModifiable;

pub fn ld<Val: CPUModifiable>(mut a: Val, b: Val) {
    a.set16(b.get16());
}

pub fn inc<Val: CPUModifiable>(val: &mut Val) {
    val.set16(val.get16() + 1);
}

pub fn dec<Val: CPUModifiable>(val: &mut Val) {
    val.set16(val.get16() - 1);
}