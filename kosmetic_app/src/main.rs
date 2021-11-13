use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::thread::sleep;
use std::time::Duration;

use kosmetic_zx::memory::*;
use kosmetic_zx::bus::*;
use kosmetic_zx::cpu::*;
use kosmetic_zx::common::*;
use kosmetic_zx::*;
use kosmetic_zx::clock::Clock;
use kosmetic_zx::ula::Ula;
use kosmetic_zx::video::VideoLayer;

#[cfg(feature = "tracing")]
fn init_logging() {
    use tracing_subscriber::layer::SubscriberExt;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_tracy::TracyLayer::new()),
    ).expect("set up the subscriber");
}

#[cfg(not(feature = "tracing"))]
fn init_logging() {}

fn main() {
    init_logging();

    let mut bus = Bus::new();

    let cpuram = cpumem::CPURam::new();
    let ularam = ulamem::ULARam::new();
    let rom = rom::Rom::new([0;0x4000]);
    let mut ula_clock = Ula::new(Some(()));
    let (cpu_clock, _) = mpsc::channel();

    let mut clock = Clock::new(cpu_clock.clone(),ula_clock.0.clone());

    bus.write().unwrap().add_device(cpuram);
    bus.write().unwrap().add_device(ularam);
    bus.write().unwrap().add_device(rom);
    bus.write().unwrap().add_device(ula_clock.1);

    loop {
        bus.write().unwrap().write(0, 2 as Byte, true);
    }
}
