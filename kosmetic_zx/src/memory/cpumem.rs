use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::common::{Address, Byte};
use crate::bus::{BusMessage, Range};

#[cfg(feature = "trace-memory")]
use tracing::*;

#[derive(Debug)]
pub struct CPURam {
    pub(crate) bytes: [Byte; 0x8000],
    pub(crate) receiver: Receiver<BusMessage>
}

impl CPURam {
    pub fn new() -> Sender<BusMessage> {
        let (tx, rx) = mpsc::channel();
        thread::spawn( move || {
            let mut ram = CPURam {
                bytes: [0; 0x8000],
                receiver: rx
            };
            ram.message_loop();
        });

        tx
    }

    fn message_loop(&mut self) {
        loop {
            match self.receiver.recv().unwrap() {
                BusMessage::MemPut(a, b, s) => {
                    self.bytes[a as usize] = b;
                    s.send(BusMessage::MemWriteOk()).unwrap();
                },
                BusMessage::MemGet(a, s) => {
                    s.send(BusMessage::MemReadOk(self.bytes[a as usize])).unwrap();
                },
                BusMessage::IOPut(_, _, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::IOGet(_, s) => s.send(BusMessage::Err).unwrap(),
                _ => {}
            }
        }
    }
}
