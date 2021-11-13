pub mod common;
pub mod bus;
pub mod memory;
pub mod cpu;
pub mod ula;
pub mod clock;
pub mod video;

#[cfg(feature = "trace-deps")]
extern crate tracing;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex, RwLock};
    use std::sync::Once;

    use crate::memory::*;
    use crate::bus::*;
    use crate::cpu::*;
    use crate::memory::cpumem::CPURam;
    use crate::memory::rom::Rom;
    use crate::memory::ulamem::ULARam;
    use crate::ula::Ula;
    use crate::video::VideoLayer;

    struct InitRet(Arc<RwLock<Bus>>, CPURam, ULARam, Rom);

    fn init() -> InitRet {
        let mut bus = crate::bus::Bus::new();

        let cpuram = CPURam::new();
        let ularam = ULARam::new();
        let rom = Rom::new([0;0x4000]);
        let ula = Ula::new(bus.clone(), None);

        bus.write().unwrap().add_device(Arc::new(RwLock::new(cpuram)));
        bus.write().unwrap().add_device(Arc::new(RwLock::new(ularam)));
        bus.write().unwrap().add_device(Arc::new(RwLock::new(rom)));
        bus.write().unwrap().add_device(Arc::new(RwLock::new(ula)));

        InitRet(bus.clone(),cpuram,ularam,rom)
    }

    #[test]
    fn cpu_ram_bus() {
        let mut devices = init();

        assert_eq!(devices.0.write().unwrap().write(0x8001, 0xFF, false), Ok(()), "Testing CPURam writing: Writing 0xFF to 0x8001");

        assert_eq!(devices.0.read().unwrap().read(0x8001, false), Ok(0xFF), "Testing CPURam reading: Reading 0x8001");

        assert_eq!(devices.0.read().unwrap().get_device_at(RWEnum::Read, 0x8001, false).unwrap().read().unwrap().get_memory().unwrap()[1], 0xFF, "Checking that the byte mentioned is actually in CPURam's memory");
    }

    #[test]
    fn ula_ram_bus() {
        let mut devices = init();

        assert_eq!(devices.0.write().unwrap().write(0x4001, 0xFF, false), Ok(()), "Testing ULARam writing: Writing 0xFF to 0x4001");

        assert_eq!(devices.0.read().unwrap().read(0x4001, false), Ok(0xFF), "Testing ULARam reading: Reading 0x4001");

        assert_eq!(devices.0.read().unwrap().get_device_at(RWEnum::Read, 0x4001, false).unwrap().read().unwrap().get_memory().unwrap()[1], 0xFF, "Checking that the byte mentioned is actually in ULARam's memory");
    }

    #[test]
    fn rom_bus() {
        let mut devices = init();

        assert_eq!(devices.0.write().unwrap().write(0x0001, 0xFF, false), Err(()), "Testing Rom writing: Writing 0xFF to 0x0001");

        assert_eq!(devices.0.read().unwrap().read(0x0001, false), Ok(0x00), "Testing Rom reading: Reading 0x8001");
    }

    #[test]
    fn rom() {
        let mut rom = Rom::new([0x7F;0x4000]);

        assert_eq!(rom.write(1, 0xFF, false), Err(()));
        assert_eq!(rom.read(1, false), Ok(0x7F));
    }

    #[test]
    fn cpu_ram() {
        let mut cpuram = CPURam::new();

        assert_eq!(cpuram.write(1, 0x7F, false), Ok(()));
        assert_eq!(cpuram.read(1, false), Ok(0x7F));
        assert_eq!(cpuram.bytes[1], 0x7F);
    }

    #[test]
    fn ula_ram() {
        let mut ularam = ULARam::new();

        assert_eq!(ularam.write(1, 0x7F, false), Ok(()));
        assert_eq!(ularam.read(1, false), Ok(0x7F));
        assert_eq!(ularam.bytes[1], 0x7F);
    }

}
