#[macro_use]
extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::atomic::{AtomicUsize, Ordering};

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
const W: u32 = 600;
const N: u32 = 260;
#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    set_panic_hook();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let canvas: web_sys::HtmlCanvasElement = document.create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_id("canvas");
    canvas.set_height(W);
    canvas.set_width(W);
    body.append_child(&canvas)?;
    log("33");
    Ok(())
}

const R: f64 = std::f64::consts::PI * 2.0 / N as f64;
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);
static mut X: f64 = 0.0;
static mut Y: f64 = 0.0;
#[wasm_bindgen]
pub fn draw(ctx: &web_sys::CanvasRenderingContext2d) -> Result<web_sys::ImageData, JsValue> {
    let count = FRAME_COUNT.fetch_add(1, Ordering::SeqCst);
    let t: f64 = count as f64 / 300.0;
    let mut bitmap: Vec<u8> = vec![0; (W * W * 4) as usize];
    let mut data = &mut bitmap;
    for i in 0..N {
        for j in 0..N {
            let ii = i as f64;
            unsafe {
                let u = (ii + Y).sin() + (R * ii + X).sin();
                let v = (ii + Y).cos() + (R * ii + X).cos();
                X = u + t;
                Y = v;
                put_dot(&mut data
                    , ((u * (N/2) as f64) as i32 + (W/2) as i32) as u32
                    , ((v * (N/2) as f64) as i32 + (W/2) as i32) as u32
                    , i as u8
                    , j as u8
                    , 100
                    , 255
                );
            }
        }
    }
    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mut data[..]),
        W, W
    )?;
    ctx.put_image_data(&image_data, 0.0, 0.0)?;
    Ok(image_data)
}

fn put_dot(data: &mut std::vec::Vec<u8>, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
    let base: usize = ((y * W + x) * 4) as usize;
    data[base    ] = r;
    data[base + 1] = g;
    data[base + 2] = b;
    data[base + 3] = a;
}