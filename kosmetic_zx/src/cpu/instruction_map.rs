use crate::cpu::Cpu;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone)]
pub enum MapObj<'a> {
    NotImplemented,
    Func(&'a dyn Fn(Arc<Mutex<Cpu>>, ) -> u16),
    SubMap(&'a [MapObj<'a>; 0xFF])
}

pub struct InstructionMap<'a> {
    pub map: [MapObj<'a>; 0xFF]
}

impl InstructionMap<'static> {
    pub fn new() -> Arc<InstructionMap<'static>> {
        let mut inst_map = InstructionMap {
            map: [MapObj::NotImplemented; 0xFF]
        };

        inst_map.map[0x0] = MapObj::Func( &| _: Arc<Mutex<Cpu>> | { 1 } );
        inst_map.map[0x1] = 

        Arc::new(inst_map)
    }
}