use std::sync::mpsc;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::thread;
use crate::common::{Byte};
use crate::bus::{BusMessage, Range};

#[cfg(feature = "trace-memory")]
use tracing::*;

#[derive(Debug)]
pub struct ULARam {
    pub(crate) bytes: [Byte; 0x4000],
    pub(crate) receiver: Receiver<BusMessage>
}

impl ULARam {
    pub fn new() -> Sender<BusMessage> {
        let (tx, rx) = bounded(128);
        thread::spawn( move || {
            let mut ula = ULARam {
                bytes: [0; 0x4000],
                receiver: rx
            };

            ula.message_loop();
        });

        tx
    }

    fn message_loop(&mut self) {
        loop {
            match self.receiver.recv().unwrap() {
                BusMessage::MemPut(a, b, s) => {
                    #[cfg(feature = "trace-memory")]
                        let _ = span!(Level::TRACE, "Write to ULAMem").enter();
                    self.bytes[a as usize] = b;
                    s.send(BusMessage::MemWriteOk).unwrap();
                },
                BusMessage::MemGet(a, _, s) => {
                    #[cfg(feature = "trace-memory")]
                        let _ = span!(Level::TRACE, "Read from ULAMem").enter();
                    s.send(BusMessage::MemReadOk(vec![self.bytes[a as usize]])).unwrap();
                },
                BusMessage::IOPut(_, _, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::IOGet(_, _, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::GetRanges(s) => {
                    #[cfg(feature = "trace-memory")]
                        let _ = span!(Level::TRACE, "Send ULARam memory ranges").enter();
                    s.send(BusMessage::RangesRet(vec![Range(0x4000,0x7FFF)],vec![Range(0x4000,0x7FFF)],vec![],vec![])).unwrap();
                },
                _ => {}
            }
        }
    }
}