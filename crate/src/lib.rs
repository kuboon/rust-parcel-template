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
const W: u32 = 540;
const N: u32 = 100;
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
    Ok(())
}

const R: f64 = std::f64::consts::PI * 2.0 / N as f64;
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);
static mut x: f64 = 0.0;
static mut y: f64 = 0.0;
#[wasm_bindgen]
pub fn draw(ctx: &web_sys::CanvasRenderingContext2d) {
    let count = FRAME_COUNT.fetch_add(1, Ordering::SeqCst);
    let t: f64 = count as f64 / 10.0;
    ctx.clear_rect(0.0, 0.0, W as f64, W as f64);
    for i in 0..N {
        for j in 0..N {
            let ii = i as f64;
            ctx.set_fill_style(&JsValue::from_str(&format!("rgb({},{},{})",i,j,255)));
            unsafe {
                let u = (ii + y).sin() + (R * ii + x).sin();
                let v = (ii + y).cos() + (R * ii + x).cos();
                x = u + t;
                y = v;
                ctx.fill_rect(u * (N/2) as f64 + (W/2) as f64, y * (N/2) as f64 + (W/2) as f64, 2.0, 2.0);
            }
        }
    }
}