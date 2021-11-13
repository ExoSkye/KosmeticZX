use std::sync::{Mutex, Arc};
use sdl2::{AudioSubsystem, Sdl, VideoSubsystem};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct VideoLayer {
    pub ctx: Arc<Mutex<Sdl>>,
    pub vid_sub_sys: Arc<Mutex<VideoSubsystem>>,
    pub audio_sub_sys: Arc<Mutex<AudioSubsystem>>,
    pub canvas: Arc<Mutex<Canvas<Window>>>
}

impl VideoLayer {
    pub fn new() -> Arc<Mutex<VideoLayer>> {
        let ctx = sdl2::init().expect("Couldn't init SDL");
        let vid_sub_sys = ctx.video().expect("Couldn't get SDL VideoSubsystem");
        let audio_sub_sys = ctx.audio().expect("Couldn't get SDL AudioSubsystem");
        let window = vid_sub_sys.window("KosmeticZX", 256, 192)
            .position_centered()
            .build().expect("Couldn't build window");

        Arc::new(Mutex::new(VideoLayer {
            ctx: Arc::new(Mutex::new(ctx)),
            vid_sub_sys: Arc::new(Mutex::new(vid_sub_sys)),
            audio_sub_sys: Arc::new(Mutex::new(audio_sub_sys)),
            canvas: Arc::new(Mutex::new(window.into_canvas().accelerated().build().expect("Couldn't build canvas")))
        }))
    }
}