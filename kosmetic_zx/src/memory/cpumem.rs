use std::sync::mpsc;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::thread;
use crate::common::{Byte};
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
        let (tx, rx) = bounded(128);
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
                    #[cfg(feature = "trace-memory")]
                        let _ = span!(Level::TRACE, "Write to CPUMem").enter();
                    self.bytes[a as usize] = b;
                    s.send(BusMessage::MemWriteOk).unwrap();
                },
                BusMessage::MemGet(a, s) => {
                    #[cfg(feature = "trace-memory")]
                        let _ = span!(Level::TRACE, "Read from CPUMem").enter();
                    s.send(BusMessage::MemReadOk(self.bytes[a as usize])).unwrap();
                },
                BusMessage::IOPut(_, _, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::IOGet(_, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::GetRanges(s) => {
                    #[cfg(feature = "trace-memory")]
                        let _ = span!(Level::TRACE, "Send CPUMem memory ranges").enter();
                    s.send(BusMessage::RangesRet(vec![Range(0x8000,0xFFFF)],vec![Range(0x8000,0xFFFF)],vec![],vec![])).unwrap();
                },
                _ => {}
            }
        }
    }
}
