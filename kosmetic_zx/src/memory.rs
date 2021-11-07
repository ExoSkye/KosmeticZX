use crate::common::{Address, Byte};
use crate::bus::{MmioDevice, Range};

#[cfg(feature = "trace-memory")]
use tracing::*;

#[derive(Clone, Copy, Debug)]
pub struct CPURam {
    pub(crate) bytes: [Byte; 0x8000]
}

impl CPURam {
    pub fn new() -> CPURam {
        crate::init_logging();
        CPURam {
            bytes: [0; 0x8000]
        }
    }
}

impl MmioDevice for CPURam {
    #[cfg_attr(feature = "trace-memory", instrument(name = "Write to CPURam", skip_all))]
    fn write(&mut self, address: Address, data: Byte) -> Result<(), ()> {
        if address < 0x8000 {
            self.bytes[address as usize] = data;
            return Ok(());
        }
        else {
            return Err(());
        }
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Read from CPURam", skip_all))]
    fn read(&self, address: Address) -> Result<Byte, ()> {
        if address < 0x8000 {
            return Ok(self.bytes[address as usize]);
        }
        else {
            return Err(());
        }
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get read ranges for CPURam", skip_all))]
    fn get_read_ranges(&self) -> Vec<Range> {
        return vec![Range(0x8000, 0xFFFF)];
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get write ranges for CPURam", skip_all))]
    fn get_write_ranges(&self) -> Vec<Range> {
        return vec![Range(0x8000, 0xFFFF)];
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get direct memory from CPURam", skip_all))]
    fn get_memory(&self) -> Option<Vec<Byte>> {
        return Some(Vec::from(self.bytes));
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rom {
    pub(crate) contents: [Byte; 0x4000]
}

impl MmioDevice for Rom {
    #[cfg_attr(feature = "trace-memory", instrument(name = "Try to write to ROM?", skip_all))]
    fn write(&mut self, _address: Address, _data: Byte) -> Result<(), ()> {
        Err(())
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Read from ROM", skip_all))]
    fn read(&self, address: Address) -> Result<Byte, ()> {
        if address < 0x4000 {
            return Ok(self.contents[address as usize]);
        } else {
            return Err(());
        }
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get read ranges for ROM", skip_all))]
    fn get_read_ranges(&self) -> Vec<Range> {
        return vec![Range(0x0000, 0x3FFF)];
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get write ranges for ROM", skip_all))]
    fn get_write_ranges(&self) -> Vec<Range> {
        return vec![Range(0x0000, 0x3FFF)];
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get direct memory from ROM", skip_all))]
    fn get_memory(&self) -> Option<Vec<Byte>> {
        return Some(Vec::from(self.contents));
    }
}

impl Rom {
    pub fn new(contents: [Byte; 0x4000]) -> Rom {
        #[cfg(feature = "trace-memory")]
            crate::init_logging();
        Rom {
            contents
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ULARam {
    pub(crate) bytes: [Byte; 0x4000]
}

impl ULARam {
    pub fn new() -> ULARam {
        #[cfg(feature = "trace-memory")]
            crate::init_logging();
        ULARam {
            bytes: [0; 0x4000]
        }
    }
}

impl MmioDevice for ULARam {
    #[cfg_attr(feature = "trace-memory", instrument(name = "Write to ULARam", skip_all))]
    fn write(&mut self, address: Address, data: Byte) -> Result<(), ()> {
        if address < 0x4000 {
            self.bytes[address as usize] = data;
            return Ok(());
        }
        else {
            return Err(());
        }
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Read from ULARam", skip_all))]
    fn read(&self, address: Address) -> Result<Byte, ()> {
        if address < 0x4000 {
            return Ok(self.bytes[address as usize]);
        }
        else {
            return Err(());
        }
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get read ranges for ULARam", skip_all))]
    fn get_read_ranges(&self) -> Vec<Range> {
        return vec![Range(0x4000, 0x7FFF)];
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get read ranges for ULARam", skip_all))]
    fn get_write_ranges(&self) -> Vec<Range> {
        return vec![Range(0x4000, 0x7FFF)];
    }

    #[cfg_attr(feature = "trace-memory", instrument(name = "Get read ranges for ULARam", skip_all))]
    fn get_memory(&self) -> Option<Vec<Byte>> {
        return Some(Vec::from(self.bytes));
    }
}