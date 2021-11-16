#[cfg(feature = "hash-mem-map")]
use {
    std::collections::HashMap,
    HashMap as memMap,
};

#[cfg(feature = "btree-mem-map")]
use {
    std::collections::BTreeMap,
    BTreeMap as memMap,
};

#[cfg(feature = "index-mem-map")]
use {
    indexmap::IndexMap,
    IndexMap as memMap,
};

use crate::common::{Address, Byte};
use std::fmt::Debug;
use std::sync::{Arc, mpsc, RwLock};
use crossbeam_channel::{Receiver, Sender, bounded};
use std::thread;

#[cfg(feature = "trace-bus")]
use tracing::*;

#[derive(PartialEq, Debug)]
pub enum RWEnum {
    Read,
    Write,
}

#[derive(Debug)]
pub enum BusMessage {
    AddDevice(Sender<BusMessage>, Sender<BusMessage>),
    AddDeviceOk,
    GetRanges(Sender<BusMessage>),
    RangesRet(Vec<Range>, Vec<Range>, Vec<Range>, Vec<Range>),
    IOGet(Address, Sender<BusMessage>),
    MemGet(Address, Sender<BusMessage>),
    IOPut(Address, Byte, Sender<BusMessage>),
    MemPut(Address, Byte, Sender<BusMessage>),
    IOWriteOk,
    IOReadOk(Byte),
    MemWriteOk,
    MemReadOk(Byte),
    Err
}

#[derive(Debug, Clone)]
pub struct Range(pub Address, pub Address);

#[derive(Debug, Clone)]
pub struct MapEntry {
    pub(crate) device: Sender<BusMessage>,
    pub(crate) range: Range,
}

#[derive(Debug)]
pub struct Bus {
    pub(crate) read_ranges: memMap<Address, MapEntry>,
    pub(crate) write_ranges: memMap<Address, MapEntry>,
    pub(crate) io_read_ranges: memMap<Address, MapEntry>,
    pub(crate) io_write_ranges: memMap<Address, MapEntry>,
    pub receiver: Receiver<BusMessage>
}

impl Bus {
    #[cfg_attr(feature = "trace-bus", instrument(name = "Create Bus", skip_all))]
    pub fn new() -> Sender<BusMessage> {
        let (tx, rx) = bounded(128);

        thread::spawn(move || {
            let mut bus = Bus {
                read_ranges: memMap::new(),
                write_ranges: memMap::new(),
                io_read_ranges: memMap::new(),
                io_write_ranges: memMap::new(),
                receiver: rx
            };

            loop {
                match bus.receiver.recv().unwrap() {
                    BusMessage::IOGet(a, s) => {
                        match bus.read(a,true) {
                            Ok(b) => {s.send(BusMessage::IOReadOk(b)).unwrap()}
                            Err(_) => {s.send(BusMessage::Err).unwrap()}
                        }
                    }
                    BusMessage::MemGet(a, s) => {
                        match bus.read(a,false) {
                            Ok(b) => {s.send(BusMessage::MemReadOk(b)).unwrap()}
                            Err(_) => {s.send(BusMessage::Err).unwrap()}
                        }
                    }
                    BusMessage::IOPut(a, b, s) => {
                        match bus.write(a, b, true) {
                            Ok(_) => {s.send(BusMessage::IOWriteOk).unwrap()}
                            Err(_) => {s.send(BusMessage::Err).unwrap()}
                        }

                    }
                    BusMessage::MemPut(a, b, s) => {
                        match bus.write(a, b, false) {
                            Ok(_) => {s.send(BusMessage::MemWriteOk).unwrap()}
                            Err(_) => {s.send(BusMessage::Err).unwrap()}
                        }
                    }
                    BusMessage::AddDevice(d,s) => {
                        bus.add_device(d);
                        s.send(BusMessage::AddDeviceOk).unwrap();
                    }
                    BusMessage::Err => {}
                    _ => {}
                }
            }
        });

        tx.clone()
    }

    #[cfg_attr(
        feature = "trace-bus",
        instrument(name = "Add device to bus", skip_all)
    )]
    pub fn add_device(&mut self, device: Sender<BusMessage>) {
        let (tx, rx) = bounded(1);
        device.send(BusMessage::GetRanges(tx)).unwrap();
        let ranges = match rx.recv().unwrap() {
            BusMessage::RangesRet(a, b, c, d) => ( a, b, c, d ),
            _ => panic!("Got unexpected message")
        };

        for range in ranges.0 {
            self.read_ranges.insert(
                range.0,
                MapEntry {
                    device: device.clone(),
                    range,
                },
            );
        }

        for range in ranges.1 {
            self.write_ranges.insert(
                range.0,
                MapEntry {
                    device: device.clone(),
                    range,
                },
            );
        }

        for range in ranges.2 {
            self.io_read_ranges.insert(
                range.0,
                MapEntry {
                    device: device.clone(),
                    range,
                },
            );
        }

        for range in ranges.3 {
            self.io_write_ranges.insert(
                range.0,
                MapEntry {
                    device: device.clone(),
                    range,
                },
            );
        }
    }

    #[cfg_attr(feature = "trace-bus", instrument(name = "Write to bus", skip_all))]
    pub fn write(&mut self, address: Address, data: Byte, io_bus: bool) -> Result<(), ()> {
        for (key, val) in if io_bus == true {
            self.io_write_ranges.iter_mut()
        } else {
            self.write_ranges.iter_mut()
        } {
            if address >= *key {
                if address < val.range.1 {
                    let (tx,rx) = bounded(1);
                    val.device.send(if io_bus {
                        BusMessage::IOPut(address - key, data, tx)
                    } else {
                        BusMessage::MemPut(address - key, data, tx)
                    }).unwrap();
                    let ret = rx.recv().unwrap();
                    return match ret {
                        BusMessage::IOWriteOk => Ok(()),
                        BusMessage::MemWriteOk => Ok(()),
                        _ => Err(())
                    }
                }
            }
        }
        Err(())
    }

    #[cfg_attr(feature = "trace-bus", instrument(name = "Read from bus", skip_all))]
    pub fn read(&self, address: Address, io_bus: bool) -> Result<Byte, ()> {
        for (key, val) in if io_bus == true {
            self.io_read_ranges.iter()
        } else {
            self.read_ranges.iter()
        } {
            if address >= *key {
                if address < val.range.1 {
                    let (tx,rx) = bounded(1);
                    val.device.send(if io_bus {
                        BusMessage::IOGet(address - key, tx)
                    } else {
                        BusMessage::MemGet(address - key, tx)
                    }).unwrap();

                    let ret = rx.recv().unwrap();

                    return match ret {
                        BusMessage::IOReadOk(data) => Ok(data),
                        BusMessage::MemReadOk(data) => Ok(data),
                        _ => Err(())
                    }
                }
            }
        }
        Err(())
    }
}
