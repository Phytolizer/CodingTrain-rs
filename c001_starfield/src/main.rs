use sdl2::{event::Event, pixels::Color};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("SDL_Init error: {0}")]
    SdlInit(String),
    #[error("SDL_InitSubSystem error: {0}")]
    SdlInitSubSystem(String),
    #[error("SDL_CreateWindow error: {0}")]
    SdlWindowBuild(#[from] sdl2::video::WindowBuildError),
    #[error("SDL event pump error: {0}")]
    SdlEventPump(String),
    #[error("SDL_CreateRenderer error: {0}")]
    SdlRendererBuild(sdl2::IntegerOrSdlError),
}

type Result<T> = std::result::Result<T, Error>;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<()> {
    let sdl = sdl2::init().map_err(Error::SdlInit)?;
    let video = sdl.video().map_err(Error::SdlInitSubSystem)?;
    let window = video.window("Starfield", WIDTH, HEIGHT).build()?;
    let mut event_pump = sdl.event_pump().map_err(Error::SdlEventPump)?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(Error::SdlRendererBuild)?;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    }

    Ok(())
}
