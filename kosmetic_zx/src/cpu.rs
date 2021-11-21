#![allow(non_snake_case)]

pub mod instruction_map;
pub mod instructions;

use crate::common::{Address, Byte};
use crossbeam_channel::{bounded, Receiver, Sender};

use crate::bus::{BusMessage};
use crate::clock::{ClockMessage};

use std::sync::{Arc, Mutex};
use std::thread;

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

pub struct ComboRegister {
    high: Arc<Byte>,
    low: Arc<Byte>
}

impl ComboRegister {
    fn new(high: Arc<Byte>, low: Arc<Byte>) -> ComboRegister {
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
    AF: Option<ComboRegister>,
    BC: Option<ComboRegister>,
    DE: Option<ComboRegister>,
    HL: Option<ComboRegister>,
    clock_rx: Receiver<ClockMessage>,
    clock_tx: Sender<ClockMessage>,
    bus_tx: Sender<BusMessage>,
    inst_map: Arc<instruction_map::InstructionMap<'a>>
}

impl Cpu<'static> {
    pub fn new(bus_sender: Sender<BusMessage>) -> (Sender<ClockMessage>, Receiver<ClockMessage>)  {
        let (clock_held_tx, clock_rx) = bounded(128);
        let (clock_tx, clock_held_rx) = bounded(128);

        thread::spawn( move || {
            let mut cpu = Cpu {
                A: 0,
                B: 0,
                C: 0,
                D: 0,
                E: 0,
                F: 0,
                H: 0,
                L: 0,
                I: 0,
                SP: 0,
                PC: 0,
                IX: 0,
                IY: 0,
                sA: 0,
                sB: 0,
                sC: 0,
                sD: 0,
                sE: 0,
                sF: 0,
                sH: 0,
                sL: 0,
                AF: None,
                BC: None,
                DE: None,
                HL: None,
                clock_rx,
                clock_tx,
                bus_tx: bus_sender,
                inst_map: instruction_map::InstructionMap::new()
            };

            cpu.AF = Some(ComboRegister::new(Arc::new(cpu.A), Arc::new(cpu.F)));
            cpu.BC = Some(ComboRegister::new(Arc::new(cpu.B), Arc::new(cpu.C)));
            cpu.DE = Some(ComboRegister::new(Arc::new(cpu.D), Arc::new(cpu.E)));
            cpu.HL = Some(ComboRegister::new(Arc::new(cpu.H), Arc::new(cpu.L)));

            cpu.loop_thing()
        });

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
        self.bus_tx.send(BusMessage::MemGet(self.PC, 2, tx));
        let ret = rx.recv().unwrap();

        match ret {
            BusMessage::MemReadOk(b) => self.inst_map.get((b[0] as Address) >> 8 | (b[1] as Address))(self),
            _ => { panic!("Couldn't read from bus"); }
        };
    }
}