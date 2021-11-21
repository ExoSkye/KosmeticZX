use std::sync::{Mutex, Arc};

#[cfg(not(feature = "no-video"))]
    use sdl2::{AudioSubsystem, EventPump, Sdl, VideoSubsystem};
    use sdl2::render::Canvas;
    use sdl2::video::Window;

#[cfg(not(feature = "no-video"))]
    pub struct VideoLayer {
        pub ctx: Arc<Mutex<Sdl>>,
        pub vid_sub_sys: Arc<Mutex<VideoSubsystem>>,
        pub audio_sub_sys: Arc<Mutex<AudioSubsystem>>,
        pub canvas: Arc<Mutex<Canvas<Window>>>,
        pub event_pump: Arc<Mutex<EventPump>>
    }

#[cfg(not(feature = "no-video"))]
    impl VideoLayer {
        pub fn new() -> Arc<Mutex<VideoLayer>> {
            let ctx = sdl2::init().expect("Couldn't init SDL");
            let vid_sub_sys = ctx.video().expect("Couldn't get SDL VideoSubsystem");
            let audio_sub_sys = ctx.audio().expect("Couldn't get SDL AudioSubsystem");
            let window = vid_sub_sys.window("KosmeticZX", 352, 315)
                .position_centered()
                .build().expect("Couldn't build window");
            let event_pump = ctx.event_pump().expect("Couldn't get event pump");

            Arc::new(Mutex::new(VideoLayer {
                ctx: Arc::new(Mutex::new(ctx)),
                vid_sub_sys: Arc::new(Mutex::new(vid_sub_sys)),
                audio_sub_sys: Arc::new(Mutex::new(audio_sub_sys)),
                canvas: Arc::new(Mutex::new(window.into_canvas().accelerated().build().expect("Couldn't build canvas"))),
                event_pump: Arc::new(Mutex::new(event_pump))
            }))
        }
    }

#[cfg(feature = "no-video")]
    pub struct VideoLayer {}
    
#[cfg(feature = "no-video")]
    impl VideoLayer {
        pub fn new() -> Arc<Mutex<VideoLayer>> {
            Arc::new(Mutex::new(VideoLayer {}))
        }
    }