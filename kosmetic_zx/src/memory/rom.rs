use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::common::{Address, Byte};
use crate::bus::{BusMessage, Range};

#[cfg(feature = "trace-memory")]
use tracing::*;

#[derive(Debug)]
pub struct Rom {
    pub(crate) contents: [Byte; 0x4000],
    pub(crate) receiver: Receiver<BusMessage>
}

impl Rom {
    pub fn new(contents: [Byte; 0x4000]) -> Sender<BusMessage> {
        let (tx, rx) = mpsc::channel();

        thread::spawn( move || {
            let mut rom = Rom {
                contents,
                receiver: rx
            };

            rom.message_loop();
        });

        tx
    }

    fn message_loop(&mut self) {
        loop {
            match self.receiver.recv().unwrap() {
                BusMessage::MemPut(_, _, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::MemGet(a, s) => {
                    s.send(BusMessage::MemReadOk(self.contents[a as usize])).unwrap();
                },
                BusMessage::IOPut(_, _, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::IOGet(_, s) => s.send(BusMessage::Err).unwrap(),
                _ => {}
            }
        }
    }
}