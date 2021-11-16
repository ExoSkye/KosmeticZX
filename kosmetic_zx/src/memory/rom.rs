use std::sync::mpsc;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::thread;
use crate::common::{Byte};
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
        let (tx, rx) = bounded(128);

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
                    #[cfg(feature = "trace-memory")]
                        let _ = span!(Level::TRACE, "Read from Rom").enter();
                    s.send(BusMessage::MemReadOk(self.contents[a as usize])).unwrap();
                },
                BusMessage::IOPut(_, _, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::IOGet(_, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::GetRanges(s) => {
                    #[cfg(feature = "trace-memory")]
                        let _ = span!(Level::TRACE, "Send ROM memory ranges").enter();
                    s.send(BusMessage::RangesRet(vec![Range(0x0000,0x3FFF)],vec![Range(0x0000,0x3FFF)],vec![],vec![])).unwrap();
                },
                _ => {}
            }
        }
    }
}