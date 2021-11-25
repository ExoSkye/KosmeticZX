#![allow(non_snake_case)]

pub mod instructions;

use crate::common::{Address, Byte};
use crossbeam_channel::{bounded, Receiver, Sender};

use crate::bus::{BusMessage};
use crate::clock::{ClockMessage};

use std::thread;

use self::instructions::meta_instructions::{bytes_to_dword, dword_to_bytes};

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



pub struct Cpu {
    count: u16,
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
    clock_rx: Receiver<ClockMessage>,
    clock_tx: Sender<ClockMessage>,
    bus_tx: Sender<BusMessage>
}

impl Cpu {
    pub fn new(bus_sender: Sender<BusMessage>) -> (Sender<ClockMessage>, Receiver<ClockMessage>)  {
        let (clock_held_tx, clock_rx) = bounded(128);
        let (clock_tx, clock_held_rx) = bounded(128);

        thread::spawn( move || {
            let mut cpu = Cpu {
                count: 0,
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
                clock_rx,
                clock_tx,
                bus_tx: bus_sender
            };

            cpu.loop_thing()
        });

        (clock_held_tx, clock_held_rx)
    }

    pub fn loop_thing(&mut self) {
        loop {
            let clock_msg = self.clock_rx.try_recv();

            if clock_msg.is_ok() {
                if clock_msg.unwrap() == ClockMessage::Tick {
                    if self.count == 0 {
                        self.count = self.execute();
                    }
                    else {
                        self.count -= 1;
                    }
                } else { break; }
            }
        }
    }

    pub fn execute(&mut self) -> u16 {
        let (tx, rx) = bounded(1);
        self.bus_tx.send(BusMessage::MemGet(self.PC, 4, tx));
        let ret = rx.recv().unwrap();

        return match ret {
            BusMessage::MemReadOk(b) => {
                //instructions::run(self, b.into_iter()[0..3])
                0u16
            }
            _ => { panic!("Couldn't read from bus"); }
        };
    }

    pub fn BC(&self) -> Address {
        bytes_to_dword((self.B, self.C))
    }

    pub fn AF(&self) -> Address {
        bytes_to_dword((self.A, self.F))
    }

    pub fn DE(&self) -> Address {
        bytes_to_dword((self.D, self.E))
    }

    pub fn HL(&self) -> Address {
        bytes_to_dword((self.H, self.L))
    }

    pub fn set_BC(&mut self, new: Address) {
        let bytes = dword_to_bytes(new);
        self.B = bytes.0;
        self.C = bytes.1;
    }

    pub fn set_AF(&mut self, new: Address) {
        let bytes = dword_to_bytes(new);
        self.A = bytes.0;
        self.F = bytes.1;
    }

    pub fn set_DE(&mut self, new: Address) {
        let bytes = dword_to_bytes(new);
        self.D = bytes.0;
        self.E = bytes.1;
    }

    pub fn set_HL(&mut self, new: Address) {
        let bytes = dword_to_bytes(new);
        self.H = bytes.0;
        self.L = bytes.1;
    }
}