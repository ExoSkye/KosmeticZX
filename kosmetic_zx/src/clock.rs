use std::sync::{Mutex};
use std::thread;
use std::thread::JoinHandle;
use crossbeam_channel::{Receiver, Sender};
use std::time::{Duration, Instant};
use crate::cpu::*;
use crate::ula::*;

#[derive(PartialEq)]
pub enum ClockMessage {
    Tick,
    Stop
}

#[cfg(feature = "trace-clock")]
use tracing::*;

static CLK_FREQ: u64 = 7_000_000_u64;
static CPU_DIVISOR: u32 = 2_u32;


pub struct Clock {
    cpu_clock: Sender<ClockMessage>,
    ula_clock: Sender<ClockMessage>,
    clk_comm: Receiver<ClockMessage>
}

impl Clock {
    pub fn new(cpu_clock: Sender<ClockMessage>, ula_clock: Sender<ClockMessage>, receiver: Receiver<ClockMessage>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut clk = Clock {
                cpu_clock,
                ula_clock,
                clk_comm: receiver
            };

            let sleep_dur = Duration::from_nanos(1_000_000_000_u64 / CLK_FREQ);

            let mut i: u32 = 0;
            loop {
                #[cfg(feature = "trace-clock")]
                    let _ = span!(Level::TRACE, "Clock").enter();

                //let start = Instant::now();

                if i % CPU_DIVISOR == 0 {
                    clk.cpu_clock.send(ClockMessage::Tick).unwrap();
                }

                clk.ula_clock.send(ClockMessage::Tick).unwrap();

                i += 1;

                let recv = clk.clk_comm.try_recv();
                if recv.is_ok() {
                    if recv.unwrap() == ClockMessage::Stop {
                        clk.ula_clock.send(ClockMessage::Stop).unwrap();
                        //clk.cpu_clock.send(ClockMessage::Stop);
                        std::thread::sleep(Duration::from_secs(1));
                        break;
                    }
                }

                //let end = Instant::now();

                //let diff = end.duration_since(start);

                /*while diff.as_nanos() < 142 {
                    let end1 = Instant::now();

                    thread::sleep(Duration::from_nanos(1));

                    let diff = end1.duration_since(start);
                }*/
            }
        })
    }
}