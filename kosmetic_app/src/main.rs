use std::sync::mpsc;
use std::time::Duration;

use kosmetic_zx::memory::*;
use kosmetic_zx::bus::*;
use kosmetic_zx::cpu::*;
use kosmetic_zx::common::*;
use kosmetic_zx::clock::Clock;
use kosmetic_zx::ula::Ula;

#[cfg(feature = "tracing")]
fn init_logging() {
    use tracing_subscriber::layer::SubscriberExt;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_tracy::TracyLayer::new()),
    ).expect("Couldn't set the tracing subscriber up");
}

#[cfg(not(feature = "tracing"))]
fn init_logging() {}

fn check_add_device(msg: BusMessage) {
    match msg {
        BusMessage::AddDeviceOk => return,
        _ => { panic!("Couldn't add device to bus") }
    }
}

fn main() {
    init_logging();

    let mut bus = Bus::new();

    let cpuram = cpumem::CPURam::new();
    let ularam = ulamem::ULARam::new();
    let rom = rom::Rom::new([0;0x4000]);

    let ula_clock = Ula::new(Some(()), bus.clone());
    let (cpu_clock, _) = mpsc::channel();

    let bus_channel = mpsc::channel();

    bus.send(BusMessage::AddDevice(cpuram, bus_channel.0.clone()));
    check_add_device(bus_channel.1.recv().unwrap());
    bus.send(BusMessage::AddDevice(ularam, bus_channel.0.clone()));
    check_add_device(bus_channel.1.recv().unwrap());
    bus.send(BusMessage::AddDevice(rom, bus_channel.0.clone()));
    check_add_device(bus_channel.1.recv().unwrap());
    bus.send(BusMessage::AddDevice(ula_clock.1, bus_channel.0.clone()));
    check_add_device(bus_channel.1.recv().unwrap());

    let clock = Clock::new(cpu_clock.clone(),ula_clock.0.clone(), ula_clock.2);

    let mut i = 1;

    loop {
        bus.send(BusMessage::IOPut(0xFE, i, bus_channel.0.clone()));
        let bus_ret = bus_channel.1.try_recv();

        if bus_ret.is_ok() {
            match bus_ret.unwrap() {
                BusMessage::IOWriteOk => {},
                _ => {
                    println!("Failed to write border colour");
                    break;
                }
            }
        }

        std::thread::sleep(Duration::from_micros(100));
        i += 1;
        if i == 8 {
            i = 0;
        }
    }
}
