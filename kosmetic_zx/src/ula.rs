use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Instant};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::bus::{BusMessage, Range};
use crate::clock::{ClockMessage};
use crate::common::{Rect, Vec2, Byte};
use crate::video::VideoLayer;

#[cfg(feature = "trace-ula")]
use tracing::*;

static BORDER_AREA: Rect = Rect { x: 96, y: 16, w: 352, h: 315 };
static SCREEN_AREA: Rect = Rect { x: BORDER_AREA.x + 48, y: BORDER_AREA.y + 48, w: 256, h: 192 };

pub struct Ula {
    bus_control_tx: Sender<BusMessage>,
    bus_rx: Receiver<BusMessage>,
    video_layer: Option<Arc<Mutex<VideoLayer>>>,
    last_refresh: Instant,
    border_color: Color,
    render_pos: Vec2,
    clock_rx: Receiver<ClockMessage>,
    clock_tx: Sender<ClockMessage>
}

impl Ula {
    pub fn new(video_layer: Option<()>, bus_sender: Sender<BusMessage>) -> (Sender<ClockMessage>, Sender<BusMessage>, Receiver<ClockMessage>) {
        let (clock_held_tx, clock_rx) = mpsc::channel();
        let (bus_tx, bus_rx) = mpsc::channel();
        let (clock_tx, clock_held_rx) = mpsc::channel();

        thread::spawn( move || {
            let mut ula = Ula {
                bus_control_tx: bus_sender.clone(),
                bus_rx,
                video_layer: if video_layer.is_some() { Some(VideoLayer::new()) } else { None },
                last_refresh: Instant::now(),
                border_color: Color::RGB(0, 0, 0),
                render_pos: Vec2::new(0, 0),
                clock_rx,
                clock_tx
            };

            ula.loop_thing()
        });

        (clock_held_tx, bus_tx, clock_held_rx)
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
        #[cfg(feature = "trace-ula")]
            let _ = span!(Level::TRACE, "Run ULA Event loop").enter();
        if self.video_layer.is_some() {
            if self.inside(SCREEN_AREA.x, SCREEN_AREA.y, SCREEN_AREA.w, SCREEN_AREA.h, self.render_pos.x, self.render_pos.y, 1, 1) {

            } else if self.inside(BORDER_AREA.x, BORDER_AREA.y, BORDER_AREA.w, BORDER_AREA.h, self.render_pos.x, self.render_pos.y, 1, 1) {
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap()
                    .set_draw_color(self.border_color);
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap()
                    .draw_point(Point::new((self.render_pos.x - BORDER_AREA.x) as i32, (self.render_pos.y - BORDER_AREA.y) as i32)).unwrap();
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
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap()
                    .present();
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap()
                    .set_draw_color(self.convert_color(0));
                self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").canvas.lock().unwrap()
                    .clear();
            }

            for event in self.video_layer.as_ref().unwrap().lock().expect("Couldn't unlock write lock for canvas").event_pump.lock().unwrap().poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => self.clock_tx.send(ClockMessage::Stop).unwrap(),
                    _ => {}
                }
            }
        }
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

    fn inside(&self, x1: u16, y1: u16, w1: u16, h1: u16,
              x2: u16, y2: u16, w2: u16, h2: u16) -> bool {
        x2 > x1 && y2 > y1 && x2 + w2 < x1 + w1 && y2 + h2 < y1 + h1
    }

    fn check_message(&mut self) {
        let msg = self.bus_rx.try_recv();
        if msg.is_ok() {
            match msg.unwrap() {
                BusMessage::MemPut(_, _, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::MemGet(_, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::IOPut(_, b, s) => {
                    #[cfg(feature = "trace-ula")]
                        let _ = span!(Level::TRACE, "Write to ULA Registers").enter();
                    self.border_color = self.convert_color(b);
                    s.send(BusMessage::IOWriteOk).unwrap();
                },
                BusMessage::IOGet(_, s) => s.send(BusMessage::Err).unwrap(),
                BusMessage::GetRanges(s) => {
                    #[cfg(feature = "trace-ula")]
                        let _ = span!(Level::TRACE, "Send ULA memory-mapped ranges").enter();
                    s.send(BusMessage::RangesRet(vec![], vec![], vec![Range(0xFE, 0xFF)], vec![Range(0xFE, 0xFF)])).unwrap();
                },
                _ => {}
            }
        }
    }
}