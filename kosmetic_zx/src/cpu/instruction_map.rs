use crate::cpu::Cpu;
use crate::common::{Address, Byte};
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone)]
pub enum MapObj<'a> {
    NotImplemented,
    Func(&'a dyn Fn(&mut Cpu, ) -> u16),
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

        inst_map.map[0x0] = MapObj::Func( &| _: &mut Cpu | { 1 } );
        //inst_map.map[0x1] =

        Arc::new(inst_map)
    }

    pub fn get(&self, idx: Address) -> &'static dyn Fn(&mut Cpu) -> u16 {
        self.get_submap(self.map, idx)
    }

    fn get_submap(&self, map: [MapObj<'static>; 0xFF], idx: Address) -> &'static dyn Fn(&mut Cpu) -> u16 {
        match map[(idx & 0b11111111) as usize] {
            MapObj::Func(f) => return f,
            MapObj::SubMap(m) => return self.get_submap(*m, idx << 8),
            _ => { unimplemented!() }
        }
    }
}