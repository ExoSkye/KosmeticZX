use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::bus::RWEnum::Read;
use crate::common::{Address, Byte};

#[cfg(feature = "trace-bus")]
use tracing::*;

#[derive(PartialEq, Debug)]
pub enum RWEnum {
    Read,
    Write
}

#[derive(Debug, Clone)]
pub struct Range(pub Address, pub Address);

pub trait MmioDevice: Debug {
    fn write(&mut self, address: Address, data: Byte) -> Result<(),()>;
    fn read(&self, address: Address) -> Result<Byte,()>;
    fn get_read_ranges(&self) -> Vec<Range>;
    fn get_write_ranges(&self) -> Vec<Range>;
    fn get_memory(&self) -> Option<Vec<Byte>>;
}

#[derive(Debug, Clone)]
pub struct MapEntry {
    pub(crate) device: Arc<Mutex<dyn MmioDevice>>,
    pub(crate) range: Range
}

#[derive(Debug)]
pub struct Bus {
    pub(crate) read_ranges: HashMap<Address, MapEntry>,
    pub(crate) write_ranges: HashMap<Address, MapEntry>
}

impl Bus {
    #[cfg_attr(feature = "trace-bus", instrument(name = "Create Bus", skip_all))]
    pub fn new() -> Bus {
        crate::init_logging();
        Bus {
            read_ranges: HashMap::new(),
            write_ranges: HashMap::new()
        }
    }

    #[cfg_attr(feature = "trace-bus", instrument(name = "Add device to bus", skip_all))]
    pub fn add_device<T: 'static + MmioDevice>(&mut self, device: Arc<Mutex<T>>) {
        for range in device.lock().unwrap().get_read_ranges() {
            self.read_ranges.insert(range.0, MapEntry {
                device: device.clone(),
                range
            });
        }

        for range in device.lock().unwrap().get_write_ranges() {
            self.write_ranges.insert(range.0, MapEntry {
                device: device.clone(),
                range
            });
        }
    }

    #[cfg_attr(feature = "trace-bus", instrument(name = "Get reference to bus", skip_all))]
    pub fn get_ref(self) -> Arc<Mutex<Bus>> {
        Arc::new(Mutex::new(self))
    }

    #[cfg_attr(feature = "trace-bus", instrument(name = "Write to bus", skip_all))]
    pub fn write(&mut self, address: Address, data: Byte) -> Result<(),()> {
        for key in self.write_ranges.clone().iter_mut().map(|(key, _)| { key }) {
            if address >= *key {
                if address < self.write_ranges.get(key).unwrap().range.1 {
                    return self.write_ranges.get_mut(key).unwrap().device.lock().unwrap().write(address - key, data);
                }
            }
        }
        Err(())
    }

    #[cfg_attr(feature = "trace-bus", instrument(name = "Read from bus", skip_all))]
    pub fn read(&self, address: Address) -> Result<Byte, ()> {
        for key in self.read_ranges.clone().iter().map(|(key, _)| { key }) {
            if address >= *key {
                if address < self.read_ranges.get(key).unwrap().range.1 {
                    return self.read_ranges.get(key).unwrap().device.lock().unwrap().read(address - key);
                }
            }
        }
        Err(())
    }

    #[cfg_attr(feature = "trace-bus", instrument(name = "Get device from memory address", skip_all))]
    pub fn get_device_at(&self, rw: RWEnum, address: Address) -> Option<Arc<Mutex<dyn MmioDevice>>> {
        let map = if rw == Read { self.read_ranges.clone() } else { self.write_ranges.clone() };
        for key in map.iter().map(|(key, _)| { key }) {
            if address >= *key {
                if address < map.get(key).unwrap().range.1 {
                    return Some(map.get(key).unwrap().device.clone());
                }
            }
        }
        None
    }
}

