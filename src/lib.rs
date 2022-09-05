/**
 * Entry point file = lib.rs
 */

mod utils;
mod webgl2;
mod game_of_life;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::{events::EventListener};
use crate::utils::set_panic_hook;
use crate::webgl2::starto;
use crate::utils::log;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    log("inside main");
    set_panic_hook();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p").unwrap()
        .dyn_into::<web_sys::HtmlParagraphElement>()
        .unwrap();
    // document.get_element_by_id("p");
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;

    log("before starto");
    starto()?;

    let mut closed_val = 3;

    let on_down = EventListener::new(&val, "mousedown", move |_event| {
        log("paragraph mouse down");
        closed_val = closed_val + 4;
        // web_sys::console::log_1(&"Paragrapah mousedown".into());
        {
            let mut owned_string = "closed val is ".to_owned();
            owned_string.push_str(&closed_val.to_string());
            log(&owned_string);
        }
    });
    on_down.forget();

    {
        let mut owned_string = "closed val is ".to_owned();
        owned_string.push_str(&closed_val.to_string());
        log(&owned_string);
    }

    Ok(())
}
