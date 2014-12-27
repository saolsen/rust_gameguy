// todos
// * dynamic loading
// * sound
// * input
// * fullscreen

// do color as a float instead of a u8, from 0 to 1
// find out if float is f32 or f64, I think it's 32 but is 64 just as fast?

// Also, I think I could have my array be of a 32 bit type and it would still work.
// just need to pass the right pitch to sdl since it reads the raw array.
// use floats for the r g and b values.

// see if rust has a way to round a float to an int

extern crate sdl2;

use sdl2::video::{WindowPos, Window, OPENGL, FullscreenType};
use sdl2::render::{Renderer, RenderDriverIndex,
                   ACCELERATED, TextureAccess};
use sdl2::timer::{get_performance_counter, get_performance_frequency,
                  get_ticks, delay};
use sdl2::event::{Event, poll_event};
use sdl2::surface::{Surface};
use sdl2::pixels::{PixelFormatFlag};
use sdl2::keycode::{KeyCode};

const width: int = 960;
const height: int = 540;
const bytes_per_pixel: uint = 4u;
const pitch: uint = bytes_per_pixel * width as uint;
const buffer_size: uint = pitch * height as uint;

type PixelBuffer = [u8, ..buffer_size];

// Shared Types between the platform layer and the game.
// question: Can I have an immutable gamestate? Is that something I would want?
struct Gamestate {
    width : f64,
}

// THIS IS THE GAME CODE THAT GOES IN THE GAME AND NOT HERE NO NO NOT HERE

// todo(stephen): check out rusts parametric stuff to make this one function.
//                or even better see if this is in rust's standard library.
fn round_to_u8(n: f64) -> u8 {
    if n >= 0.0 {
        return (n + 0.5) as u8;
    } else {
        return (n - 0.5) as u8;
    }
}

fn round_to_uint(n: f64) -> uint {
    if n >= 0.0 {
        return (n + 0.5) as uint;
    } else {
        return (n - 0.5) as uint;
    }
}

// todo(stephen): probably want the screen buffer to be a struct with the pitch and things
//                included
fn draw_rectangle(screen_buffer: &mut PixelBuffer,
                  tlx: f64, tly: f64, brx: f64, bry: f64,
                  r: f64, g: f64, b: f64) {

    // drawing with floats because we want subpixel rendering eventuially.
    let tlxu = round_to_uint(tlx);
    let tlyu = round_to_uint(tly);
    let brxu = round_to_uint(brx);
    let bryu = round_to_uint(bry);

    let rect_width = brxu - tlxu;
    let rect_height = bryu - tlyu;

    let row_offset = tlxu * bytes_per_pixel;

    // Is there a better way to do this with array slice iterators so we avoid indexing
    // bounds check overheads?

    let mut row_starts = range(0u, rect_height)
        .map(|x| (x * pitch) + row_offset);

    for row_start in row_starts {

        let mut pixel_range = range(0u, rect_width)
            .map(|x| (x * bytes_per_pixel) + row_start);
        
        for pixel in pixel_range {

            screen_buffer[pixel]    = 0;
            screen_buffer[pixel+1u] = round_to_u8(r * 255.0);
            screen_buffer[pixel+2u] = round_to_u8(g * 255.0);
            screen_buffer[pixel+3u] = round_to_u8(b * 255.0);

        }
    }
}

fn game_update_and_render(screen_buffer: &mut PixelBuffer,
                          state : &mut Gamestate,
                          dt : f64) {
    draw_rectangle(screen_buffer, 5.0, 5.0, state.width, 100.5, 1.0, 1.0, 0.0);
    state.width = state.width + 128.0 * dt;
}

// END THE GAME CODE THAT GOES IN THE GAME AND NOT HERE

fn get_seconds_elapsed(old_counter: u64, current_counter: u64) -> f64 {
    return (current_counter - old_counter) as f64 /
        get_performance_frequency() as f64;
}

fn main() {
    let running = true;

    let game_update_hz = 30.0;
    let target_seconds_per_frame = 1.0 / game_update_hz;
    let perf_count_frequency = get_performance_frequency();

    if !sdl2::init(sdl2::INIT_VIDEO) {
        panic!("Error initializing SDL")
    }

    // 1920 x 1080 x 60hz display
    // we will draw to 960 x 540 for our rendering.
    // so I'm making the window that size, sdl will handle resizing
    // and if I change over to opengl I'll figure it out then.
    let window  = match Window::new("Gameguy",
                                    WindowPos::Positioned(0),
                                    WindowPos::Positioned(0),
                                    width,
                                    height,
                                    OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // todo(stephen): do an opengl renderer eventuially.
    let renderer = match Renderer::from_window(window,
                                               RenderDriverIndex::Auto,
                                               ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };

    // todo(stephen): figure out how to make this call, I think it's needed for fullscreen.
    // SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear");
    renderer.set_logical_size(width, height);
    
    // todo(stephen): the live update reloading is basically a hard dependency
    // so I need to get it working in rust or just suck it up and switch to c.
    // I maybe should still switch to c even though I am really interested in
    // doing it in rust and having the extra safety so it doesn't just fuck up.

    let screen_texture =
        match renderer.create_texture(PixelFormatFlag::ARGB8888,
                                      TextureAccess::Streaming,
                                      width,
                                      height) {
            Ok(texture) => texture,
            Err(err)    => panic!("failed to create texture: {}", err)
        };

    let mut screen_buffer : Box<PixelBuffer> = box [0u8, ..buffer_size];
    let mut gamestate = Gamestate { width: 10.0 };
    let mut is_fullscreen = false;

    let mut last_counter = get_performance_counter();
    
    'game : loop {

        let mut event_count = 0i;

        'events : loop {
        
            match poll_event() {
                Event::None => break 'events,

                Event::Quit(_) => break 'game,

                Event::KeyDown(_, _, KeyCode::Escape, _, _, false) => {
                    println!("event: Event::KeyDown Escape, quiting");
                    break 'game
                },

                // todo(stephen): figure out why fullscreen isn't working.
                Event::KeyDown(_, the_window, KeyCode::F, _, _, false) => {
                    println!("event: Event::KeyDown F, resizing");
                    if is_fullscreen {
                        the_window.set_fullscreen(FullscreenType::FTOff);
                    } else {
                        the_window.set_fullscreen(FullscreenType::FTTrue);
                    }
                },

                Event::KeyDown(a, b, c, d, e, f) =>
                    println!("event: Event::KeyDown {},{},{},{}", a, c, d, f),

                event              => {
                    println!("event: {}", event);
                    event_count += 1
                }
            }
        }
        if event_count > 0 {
            println!("processed {} additional events", event_count);
        }

        //todo(stephen): Do I want to clear the buffer each frame before running the game?

        game_update_and_render(&mut *screen_buffer, &mut gamestate, target_seconds_per_frame);

        // delay the rest of the frame.
        // todo(stephen): see if there's a rust timer to use instead of sdls
        let seconds_elapsed =
            get_seconds_elapsed(last_counter,
                                get_performance_counter());
        
        if seconds_elapsed < target_seconds_per_frame {
            let time_to_sleep = (((target_seconds_per_frame -
                                   get_seconds_elapsed(last_counter,
                                                       get_performance_counter())
                                   )
                                  * 1000f64) - 1f64) as uint;
            if time_to_sleep > 0 {
                delay(time_to_sleep);
            }

            // wait the rest of the time until we want to render
            while (get_seconds_elapsed(last_counter,
                                       get_performance_counter()) <
                   target_seconds_per_frame) {
                // waiting
            }
        }

        let end_counter = get_performance_counter();

        // update the screen
        screen_texture.update(None, & *screen_buffer, pitch as int);
        renderer.clear();
        renderer.copy(&screen_texture, None, None);
        renderer.present();

        let seconds_elapsed = get_seconds_elapsed(last_counter, end_counter);
        last_counter = end_counter;
        
    }

    sdl2::quit();
}
