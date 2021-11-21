pub mod instruction_map;
pub mod instructions;

use crate::common::{Address, Byte};
use crossbeam_channel::{bounded, Receiver, Sender};

use crate::bus::{BusMessage};
use crate::clock::{ClockMessage};

use std::sync::{Arc, Mutex};

pub trait CPUModifiable : Copy {
    fn get8(self) -> Byte;

    fn set8(&mut self, new: Byte) -> ();

    fn get16(self) -> Address;

    fn set16(&mut self, new: Address) -> ();
}

impl CPUModifiable for Byte {
    fn get8(self) -> Byte {
        self
    }

    fn set8(&mut self, new: Byte) -> () {
        *self = new;
    }

    fn get16(self) -> Address {
        self as Address
    }

    fn set16(&mut self, new: Address) -> () {
        *self = new as Byte;
    }
}

impl CPUModifiable for Address {
    fn get8(self) -> Byte {
        self as Byte
    }

    fn set8(&mut self, new: Byte) -> () {
        *self = new as Address;
    }

    fn get16(self) -> Address {
        self
    }

    fn set16(&mut self, new: Address) -> () {
        *self = new;
    }
}

pub struct ComboRegister<'a> {
    high: &'a Byte,
    low: &'a Byte
}

impl ComboRegister<'static> {
    fn new(high: &'static Byte, low: &'static Byte) -> ComboRegister<'static> {
        ComboRegister {
            high, low
        }
    }
}

pub struct Cpu<'a> {
    A: Byte,
    B: Byte,
    C: Byte,
    D: Byte,
    E: Byte,
    F: Byte,
    H: Byte,
    L: Byte,
    I: Byte,
    SP: Address,
    PC: Address,
    IX: Address,
    IY: Address,
    sA: Byte,
    sB: Byte,
    sC: Byte,
    sD: Byte,
    sE: Byte,
    sF: Byte,
    sH: Byte,
    sL: Byte,
    AF: ComboRegister<'a>,
    BC: ComboRegister<'a>,
    DE: ComboRegister<'a>,
    HL: ComboRegister<'a>,
    clock_rx: Receiver<ClockMessage>,
    clock_tx: Sender<ClockMessage>,
    bus_tx: Sender<BusMessage>,
    inst_map: instruction_map::InstructionMap<'a>
}

impl Cpu<'static> {
    fn new(bus_sender: Sender<BusMessage>) -> (Sender<ClockMessage>, Receiver<ClockMessage>)  {
        let (clock_held_tx, clock_rx) = bounded(128);
        let (clock_tx, clock_held_rx) = bounded(128);

        (clock_held_tx, clock_held_rx)
    }

    pub fn loop_thing(&mut self) {
        loop {
            let clock_msg = self.clock_rx.try_recv();

            if clock_msg.is_ok() {
                if clock_msg.unwrap() == ClockMessage::Tick {
                    self.execute();
                } else { break; }
            }
        }
    }

    pub fn execute(&mut self) {
        let (tx, rx) = bounded(1);
        self.bus_tx.send(BusMessage::MemGet(self.PC, tx));
        let ret = rx.recv().unwrap();

        match ret {
            BusMessage::MemReadOk(b) => self.inst_map.get(b)(Arc::new(Mutex::new(self))),
            _ => {} 
        }

        
    }
}