use std::sync::{Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use crate::cpu::*;
use crate::ula::*;

#[derive(PartialEq)]
pub enum ClockMessage {
    Tick,
    Stop
}

static CLK_FREQ: u64 = 7_000_000_u64;
static CPU_DIVISOR: u32 = 2_u32;


pub struct Clock {
    cpu_clock: Sender<ClockMessage>,
    ula_clock: Sender<ClockMessage>,
    stop: Mutex<()>
}

impl Clock {
    pub fn new(cpu_clock: Sender<ClockMessage>, ula_clock: Sender<ClockMessage>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut clk = Clock {
                cpu_clock,
                ula_clock,
                stop: Mutex::new(())
            };

            let mut i: u32 = 0;
            loop {
                //if i % CPU_DIVISOR == 0 {
                //    self.cpu_clock.send(ClockMessage::Tick).unwrap();
                //}

                clk.ula_clock.send(ClockMessage::Tick).unwrap();

                //std::thread::sleep(Duration::from_nanos(1_000_000_000_u64 / CLK_FREQ));

                i += 1;

                if clk.stop.try_lock().is_err() {
                    break;
                }
            }
        })
    }
}