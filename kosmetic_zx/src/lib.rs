use std::sync::Once;

pub mod common;
pub mod bus;
pub mod memory;
pub mod cpu;
pub mod ula;

#[cfg(feature = "trace-deps")]
extern crate tracing;
#[cfg(feature = "trace-deps")]
extern crate tracing_subscriber;
#[cfg(feature = "trace-deps")]
extern crate tracing_chrome;

static INIT: Once = Once::new();

#[cfg(feature = "trace-deps")]
fn init_logging() {
    INIT.call_once( || {
        use tracing_chrome::ChromeLayerBuilder;
        use tracing_subscriber::{registry::Registry, prelude::*};

        let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
    });
}

#[cfg(not(feature = "trace-deps"))]
fn init_logging() {}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::sync::Once;

    use crate::memory::*;
    use crate::bus::*;
    use crate::cpu::*;

    struct InitRet(Bus, CPURam, ULARam, Rom);

    fn init() -> InitRet {
        let mut bus = crate::bus::Bus::new();

        let cpuram = CPURam::new();
        let ularam = ULARam::new();
        let rom = Rom::new([0;0x4000]);

        bus.add_device(Arc::new(Mutex::new(cpuram)));
        bus.add_device(Arc::new(Mutex::new(ularam)));
        bus.add_device(Arc::new(Mutex::new(rom)));

        InitRet(bus,cpuram,ularam,rom)
    }

    #[test]
    fn cpu_ram_bus() {
        let mut devices = init();

        assert_eq!(devices.0.write(0x8001, 0xFF), Ok(()), "Testing CPURam writing: Writing 0xFF to 0x8001");

        assert_eq!(devices.0.read(0x8001), Ok(0xFF), "Testing CPURam reading: Reading 0x8001");

        assert_eq!(devices.0.get_device_at(RWEnum::Read, 0x8001).unwrap().lock().unwrap().get_memory().unwrap()[1], 0xFF, "Checking that the byte mentioned is actually in CPURam's memory");
    }

    #[test]
    fn ula_ram_bus() {
        let mut devices = init();

        assert_eq!(devices.0.write(0x4001, 0xFF), Ok(()), "Testing ULARam writing: Writing 0xFF to 0x4001");

        assert_eq!(devices.0.read(0x4001), Ok(0xFF), "Testing ULARam reading: Reading 0x4001");

        assert_eq!(devices.0.get_device_at(RWEnum::Read, 0x4001).unwrap().lock().unwrap().get_memory().unwrap()[1], 0xFF, "Checking that the byte mentioned is actually in ULARam's memory");
    }

    #[test]
    fn rom_bus() {
        let mut devices = init();

        assert_eq!(devices.0.write(0x0001, 0xFF), Err(()), "Testing Rom writing: Writing 0xFF to 0x0001");

        assert_eq!(devices.0.read(0x0001), Ok(0x00), "Testing Rom reading: Reading 0x8001");
    }

    #[test]
    fn rom() {
        let mut rom = Rom::new([0x7F;0x4000]);

        assert_eq!(rom.write(1, 0xFF), Err(()));
        assert_eq!(rom.read(1), Ok(0x7F));
    }

    #[test]
    fn cpu_ram() {
        let mut cpuram = CPURam::new();

        assert_eq!(cpuram.write(1, 0x7F), Ok(()));
        assert_eq!(cpuram.read(1), Ok(0x7F));
        assert_eq!(cpuram.bytes[1], 0x7F);
    }

    #[test]
    fn ula_ram() {
        let mut ularam = ULARam::new();

        assert_eq!(ularam.write(1, 0x7F), Ok(()));
        assert_eq!(ularam.read(1), Ok(0x7F));
        assert_eq!(ularam.bytes[1], 0x7F);
    }

}
