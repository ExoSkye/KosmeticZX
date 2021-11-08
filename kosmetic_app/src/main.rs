use std::sync::{Arc, Mutex};

use kosmetic_zx::memory::*;
use kosmetic_zx::bus::*;
use kosmetic_zx::cpu::*;
use kosmetic_zx::common::*;
use kosmetic_zx::*;

fn main() {
    let mut bus = Bus::new();

    let cpuram = CPURam::new();
    let ularam = ULARam::new();
    let rom = Rom::new([0;0x4000]);

    bus.add_device(Arc::new(Mutex::new(cpuram)));
    bus.add_device(Arc::new(Mutex::new(ularam)));
    bus.add_device(Arc::new(Mutex::new(rom)));

    for _ in 0..100 {
        for i in 0..0xFFFF {
            bus.write(i as Address, 0xFF);
        }
        for i in 0..0xFFFF {
            bus.read(i as Address);
        }
        for i in 0..0xFFFF {
            bus.write(i as Address, 0x00);
        }
        for i in 0..0xFFFF {
            bus.read(i as Address);
        }
    }
}
