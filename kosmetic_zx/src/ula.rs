use std::fmt::{Debug, Formatter};
use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::bus::{Bus, BusMessage, Range};
use crate::clock::{Clock, ClockMessage};
use crate::common::{Rect, Vec2, Byte, Address};
use crate::video::VideoLayer;

static FREQ: u64 = 7_000_000;
static BORDER_AREA: Rect = Rect { x: 96, y: 16, w: 352, h: 296 };
static SCREEN_AREA: Rect = Rect { x: 144, y: 64, w: 256, h: 192 };

pub struct Ula {
    bus_control_rx: Receiver<BusMessage>,
    bus_control_tx: Sender<BusMessage>,
    bus_rx: Receiver<BusMessage>,
    video_layer: Option<Arc<Mutex<VideoLayer>>>,
    last_refresh: Instant,
    border_color: sdl2::pixels::Color,
    render_pos: Vec2,
    clock_rx: Receiver<ClockMessage>
}

impl Ula {
    pub fn new(video_layer: Option<()>) -> (Sender<ClockMessage>, Sender<BusMessage>) {
        let (clock_tx, clock_rx) = mpsc::channel();
        let (bus_control_tx, bus_control_rx) = mpsc::channel();
        let (bus_tx, bus_rx) = mpsc::channel();

        thread::spawn( move || {
            let mut ula = Ula {
                bus_control_rx,
                bus_control_tx,
                bus_rx,
                video_layer: if video_layer.is_some() { Some(VideoLayer::new()) } else { None },
                last_refresh: Instant::now(),
                border_color: Color::RGB(0, 0, 0),
                render_pos: Vec2::new(0, 0),
                clock_rx
            };

            ula.loop_thing()
        });

        (clock_tx, bus_tx)
    }

    pub fn loop_thing(&mut self) {
        loop {
            let clock_msg = self.clock_rx.try_recv();

            if clock_msg.is_ok() {
                if clock_msg.unwrap() == ClockMessage::Tick {
                    self.event_loop();
                } else { break; }
            }

            self.check_message();
        }
    }

    pub fn event_loop(&mut self) {
        //let start = Instant::now();

        if self.video_layer.is_some() {
            if self.render_pos.inside(SCREEN_AREA) {
                // Do stuff
            } else if self.render_pos.inside(BORDER_AREA) {
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap().set_draw_color(self.border_color);
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap().draw_point(Point::new(self.render_pos.x as i32, self.render_pos.y as i32));
            }

            self.render_pos.x += 1;

            if self.render_pos.x >= BORDER_AREA.x + BORDER_AREA.w {
                self.render_pos.x = 0;
                self.render_pos.y += 1;
            }

            if self.render_pos.y >= BORDER_AREA.y + BORDER_AREA.h {
                self.render_pos.y = 0;
            }

            if self.render_pos == Vec2::new(0, 0) {
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap().present();
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap().set_draw_color(self.convert_color(0));
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap().clear();
            }
        }

        //let end = Instant::now();
        //thread::sleep(Duration::from_nanos(1_000_000_000 / FREQ) - (end - start));

    }

    fn convert_color(&self, data: Byte) -> Color {
        match data & 0b00000011 {
            0 => Color::RGB(0x0,0x0,0x0),
            1 => Color::RGB(0x0,0x0,0xd7),
            2 => Color::RGB(0xd7,0x0,0x0),
            3 => Color::RGB(0xd7,0x0,0xd7),
            4 => Color::RGB(0x0,0xd7,0x0),
            5 => Color::RGB(0x0,0xd7,0xd7),
            6 => Color::RGB(0xd7,0xd7,0x0),
            7 => Color::RGB(0xd7,0xd7,0xd7),
            _ => Color::RGB(0x0,0x0,0x0)
        }
    }

    fn check_message(&mut self) {
        match self.bus_rx.recv().unwrap() {
            BusMessage::MemPut(_, _, s) => s.send(BusMessage::Err).unwrap(),
            BusMessage::MemGet(_, s) => s.send(BusMessage::Err).unwrap(),
            BusMessage::IOPut(_, b, s) => {
                self.border_color = self.convert_color(b);
                s.send(BusMessage::IOWriteOk());
            },
            BusMessage::IOGet(_, s) => s.send(BusMessage::Err).unwrap(),
            BusMessage::GetRanges(s) => {
                s.send(BusMessage::RangesRet(vec![],vec![],vec![Range(0x0000,0xFFFF)],vec![Range(0x0000,0xFFFF)])).unwrap();
            },
            _ => {}
        }
    }
}

impl Debug for Ula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "");
        Ok(())
    }
}